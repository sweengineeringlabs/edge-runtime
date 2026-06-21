//! [`KafkaTaskQueue`] — Apache Kafka backed competing-consumer task queue.

use std::sync::Arc;

use bytes::Bytes;
use futures::future::BoxFuture;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{CommitMode, Consumer as _};
use rdkafka::message::Message as RdkafkaMessage;
use rdkafka::producer::{FutureProducer, FutureRecord, Producer as _};
use rdkafka::topic_partition_list::Offset;

use crate::spi::task::kafka::logging_consumer_context::{LoggingConsumer, LoggingConsumerContext};

use crate::api::QueueError;
use crate::api::Task;
use crate::api::TaskHandle;
use crate::api::TaskQueue;

/// Kafka-backed competing-consumer work queue.
///
/// Each call to [`enqueue`](KafkaTaskQueue::enqueue) publishes a message to
/// the configured Kafka topic. Each call to [`dequeue`](KafkaTaskQueue::dequeue)
/// polls the consumer group for one message:
///
/// - **ack** — commits the message offset so Kafka knows it was processed.
/// - **nack** — seeks back to the message offset so it is redelivered on the
///   next [`dequeue`](KafkaTaskQueue::dequeue) call within the same session.
///
/// If neither is called, the message is re-delivered after the consumer
/// session restarts (no committed offset for that partition).
///
/// # Rebalance contract
///
/// During a consumer-group rebalance (member join/leave/timeout), the broker
/// may revoke partition assignments mid-flight. If a partition is revoked after
/// `dequeue` returns a `TaskHandle` but before `ack`/`nack` is called:
///
/// - `ack` will attempt to commit an offset on a partition this consumer no
///   longer owns — the commit is silently dropped by Kafka.
/// - `nack` will attempt to seek on a revoked partition — rdkafka returns an
///   error which is propagated as [`QueueError::Dequeue`].
/// - The message will be redelivered to whichever consumer is assigned the
///   partition after the rebalance completes.
///
/// **Callers must be idempotent.** At-least-once delivery is guaranteed; exactly-once
/// requires external deduplication (e.g. an idempotency key in the task payload).
///
/// Rebalance events are logged at `INFO` level via [`LoggingConsumerContext`].
///
/// Requires the `kafka` Cargo feature.
pub(crate) struct KafkaTaskQueue {
    producer: FutureProducer,
    consumer: Arc<LoggingConsumer>,
    topic: String,
}

impl KafkaTaskQueue {
    /// Initialise the Kafka producer and consumer for the given topic.
    ///
    /// This call does **not** establish a network connection — rdkafka connects
    /// lazily on the first enqueue or dequeue operation.
    ///
    /// # Errors
    ///
    /// Returns [`QueueError::Connection`] if the Kafka client configuration is invalid.
    pub(crate) fn new(brokers: &str, group_id: &str, topic: &str) -> Result<Self, QueueError> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set(
                "message.timeout.ms",
                crate::core::task::KAFKA_MESSAGE_TIMEOUT_MS,
            )
            .create()
            .map_err(|e| QueueError::Connection(e.to_string()))?;

        let consumer: LoggingConsumer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("group.id", group_id)
            // Manual commit — ack() and nack() control offset progression.
            .set("enable.auto.commit", "false")
            .set("auto.offset.reset", "earliest")
            .set(
                "session.timeout.ms",
                crate::core::task::KAFKA_SESSION_TIMEOUT_MS,
            )
            .create_with_context(LoggingConsumerContext)
            .map_err(|e| QueueError::Connection(e.to_string()))?;

        consumer
            .subscribe(&[topic])
            .map_err(|e| QueueError::Connection(e.to_string()))?;

        Ok(Self {
            producer,
            consumer: Arc::new(consumer),
            topic: topic.to_owned(),
        })
    }
}

impl TaskQueue for KafkaTaskQueue {
    fn enqueue(&self, task: Task) -> BoxFuture<'_, Result<(), QueueError>> {
        let topic = self.topic.clone();
        let producer = self.producer.clone();
        Box::pin(async move {
            producer
                .send(
                    FutureRecord::to(&topic).key("").payload(&task.payload[..]),
                    std::time::Duration::from_secs(5),
                )
                .await
                .map(|_| ())
                .map_err(|(e, _)| QueueError::Enqueue(e.to_string()))
        })
    }

    fn dequeue(&self) -> BoxFuture<'_, Result<Option<TaskHandle>, QueueError>> {
        let consumer = Arc::clone(&self.consumer);
        Box::pin(async move {
            let recv_result = tokio::time::timeout(
                std::time::Duration::from_millis(crate::core::task::KAFKA_DEQUEUE_POLL_TIMEOUT_MS),
                consumer.recv(),
            )
            .await;

            let borrowed = match recv_result {
                Err(_elapsed) => return Ok(None), // no message within timeout
                Ok(Err(e)) => return Err(QueueError::Dequeue(e.to_string())),
                Ok(Ok(msg)) => msg,
            };

            // Extract all data from the borrowed message before it is dropped
            // (BorrowedMessage<'_> lifetime is tied to the consumer borrow).
            let payload = Bytes::from(borrowed.payload().unwrap_or_default().to_vec());
            let partition = borrowed.partition();
            let offset = borrowed.offset();
            let topic = borrowed.topic().to_owned();
            drop(borrowed);

            let task = Task::new(payload);

            let consumer_ack = Arc::clone(&consumer);
            let topic_ack = topic.clone();
            let ack: BoxFuture<'static, Result<(), QueueError>> = Box::pin(async move {
                use rdkafka::topic_partition_list::TopicPartitionList;
                let mut tpl = TopicPartitionList::new();
                tpl.add_partition_offset(
                    &topic_ack,
                    partition,
                    // Commit the offset AFTER this message so Kafka won't redeliver it.
                    Offset::Offset(offset + 1),
                )
                .map_err(|e| QueueError::Dequeue(e.to_string()))?;
                consumer_ack
                    .commit(&tpl, CommitMode::Async)
                    .map_err(|e| QueueError::Dequeue(e.to_string()))
            });

            let consumer_nack = Arc::clone(&consumer);
            let topic_nack = topic;
            let nack: BoxFuture<'static, Result<(), QueueError>> = Box::pin(async move {
                // Seek back to the message offset so this consumer redelivers it
                // on the next dequeue call within the same session.
                consumer_nack
                    .seek(
                        &topic_nack,
                        partition,
                        Offset::Offset(offset),
                        std::time::Duration::ZERO,
                    )
                    .map_err(|e| QueueError::Dequeue(e.to_string()))
            });

            Ok(Some(TaskHandle::new(
                task.id,
                task.payload,
                task.headers,
                ack,
                nack,
            )))
        })
    }

    fn health_check(&self) -> BoxFuture<'_, Result<(), QueueError>> {
        let producer = self.producer.clone();
        Box::pin(async move {
            tokio::task::spawn_blocking(move || {
                producer
                    .client()
                    .fetch_metadata(
                        None,
                        std::time::Duration::from_secs(
                            crate::core::task::KAFKA_HEALTH_CHECK_TIMEOUT_SECS,
                        ),
                    )
                    .map(|_| ())
                    .map_err(|e| QueueError::Connection(e.to_string()))
            })
            .await
            .map_err(|e| QueueError::Connection(format!("health check task failed: {e}")))?
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_kafka_task_queue_is_send_and_sync() {
        fn _assert<T: Send + Sync>() {}
        _assert::<KafkaTaskQueue>();
    }

    /// @covers: new
    #[tokio::test]
    async fn test_new_accepts_unreachable_broker_without_panic() {
        // rdkafka connects lazily but the subscribe call requires a tokio runtime.
        let result = KafkaTaskQueue::new("127.0.0.1:9999", "test-group", "test-topic");
        assert!(
            result.is_ok(),
            "KafkaTaskQueue::new must not fail before the first IO attempt"
        );
    }

    /// @covers: health_check
    #[tokio::test]
    async fn test_health_check_fails_for_unreachable_broker() {
        let queue = KafkaTaskQueue::new("127.0.0.1:9999", "test-group", "test-topic")
            .expect("construction succeeds before first IO");
        let result = queue.health_check().await;
        assert!(
            matches!(result, Err(QueueError::Connection(_))),
            "expected Connection error for unreachable broker, got: {result:?}"
        );
    }

    /// @covers: enqueue
    #[tokio::test]
    async fn test_enqueue_fails_for_unreachable_broker() {
        let queue = KafkaTaskQueue::new("127.0.0.1:9999", "test-group", "test-topic")
            .expect("construction succeeds before first IO");
        let result = queue.enqueue(Task::new(b"payload".as_ref())).await;
        assert!(
            matches!(result, Err(QueueError::Enqueue(_))),
            "expected Enqueue error for unreachable broker, got: {result:?}"
        );
    }
}
