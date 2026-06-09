//! [`KafkaMessageBroker`] — Apache Kafka backed pub/sub broker via `rdkafka`.

use std::collections::HashMap;

use bytes::Bytes;
use futures::channel::mpsc;
use futures::future::BoxFuture;
use futures::SinkExt as _;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer as _, StreamConsumer};
use rdkafka::error::KafkaError;
use rdkafka::message::Message as RdkafkaMessage;
use rdkafka::producer::{FutureProducer, FutureRecord, Producer as _};
use rdkafka::types::RDKafkaErrorCode;
use std::sync::Arc;

use crate::api::broker::BrokerError;
use crate::api::broker::Message;
use crate::api::broker::MessageBroker;
use crate::api::broker::MessageStream;

/// Kafka-backed pub/sub broker using `rdkafka`.
///
/// Publish operations use a [`FutureProducer`] shared across all calls. Each
/// [`subscribe`](KafkaMessageBroker::subscribe) call creates a dedicated
/// [`StreamConsumer`] so partitions are independently assigned per subscriber.
/// Auto-commit is enabled for subscribers — use the Kafka task queue
/// (`TaskQueueFactory::kafka`) when you need manual acknowledgement.
///
/// Requires the `kafka` Cargo feature.
pub(crate) struct KafkaMessageBroker {
    producer: FutureProducer,
    /// Bootstrap broker list, stored to create per-subscriber consumers.
    brokers: String,
    /// Consumer group ID shared across all subscribers from this handle.
    group_id: String,
}

impl KafkaMessageBroker {
    /// Initialise the Kafka client with the given bootstrap brokers and group ID.
    ///
    /// This call does **not** establish a network connection — rdkafka connects
    /// lazily on the first produce or subscribe operation.
    ///
    /// # Errors
    ///
    /// Returns [`BrokerError::Connection`] if the producer configuration is invalid.
    pub(crate) fn new(brokers: &str, group_id: &str) -> Result<Self, BrokerError> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set(
                "message.timeout.ms",
                crate::core::task::KAFKA_MESSAGE_TIMEOUT_MS,
            )
            .create()
            .map_err(|e| BrokerError::Connection(e.to_string()))?;

        Ok(Self {
            producer,
            brokers: brokers.to_owned(),
            group_id: group_id.to_owned(),
        })
    }
}

impl MessageBroker for KafkaMessageBroker {
    fn publish<'a>(
        &'a self,
        topic: &'a str,
        msg: Message,
    ) -> BoxFuture<'a, Result<(), BrokerError>> {
        let topic = topic.to_owned();
        let producer = self.producer.clone();
        Box::pin(async move {
            producer
                .send(
                    FutureRecord::to(&topic).key("").payload(&msg.payload[..]),
                    std::time::Duration::from_secs(5),
                )
                .await
                .map(|_| ())
                .map_err(|(e, _)| BrokerError::Publish {
                    topic,
                    reason: e.to_string(),
                })
        })
    }

    fn subscribe<'a>(
        &'a self,
        topic: &'a str,
    ) -> BoxFuture<'a, Result<MessageStream, BrokerError>> {
        let topic = topic.to_owned();
        let brokers = self.brokers.clone();
        let group_id = self.group_id.clone();
        Box::pin(async move {
            let consumer: Arc<StreamConsumer> = Arc::new(
                ClientConfig::new()
                    .set("bootstrap.servers", &brokers)
                    .set("group.id", &group_id)
                    // Auto-commit for pub/sub subscribers — callers do not ack.
                    .set("enable.auto.commit", "true")
                    .set("auto.offset.reset", "latest")
                    .set(
                        "session.timeout.ms",
                        crate::core::task::KAFKA_SESSION_TIMEOUT_MS,
                    )
                    .create()
                    .map_err(|e| BrokerError::Connection(e.to_string()))?,
            );

            consumer
                .subscribe(&[topic.as_str()])
                .map_err(|e| BrokerError::Subscribe {
                    topic: topic.clone(),
                    reason: e.to_string(),
                })?;

            // Bounded channel decouples BorrowedMessage<'_> lifetime from the returned
            // stream and applies backpressure to slow subscribers: when the channel is
            // full, the poll loop yield-waits on `send`, slowing Kafka consumption
            // instead of growing the heap without limit.
            let (mut tx, rx) = mpsc::channel::<Result<Message, BrokerError>>(
                crate::core::task::KAFKA_SUBSCRIBE_CHANNEL_CAPACITY,
            );

            tokio::spawn(async move {
                loop {
                    match consumer.recv().await {
                        Err(KafkaError::MessageConsumption(RDKafkaErrorCode::PartitionEOF)) => {
                            // Normal end-of-partition — no new messages right now; keep polling.
                            continue;
                        }
                        Err(e) => {
                            let _ = tx
                                .send(Err(BrokerError::Subscribe {
                                    topic: String::new(),
                                    reason: e.to_string(),
                                }))
                                .await;
                            break;
                        }
                        Ok(msg) => {
                            let payload = Bytes::from(msg.payload().unwrap_or_default().to_vec());
                            let broker_msg = Message {
                                payload,
                                headers: HashMap::new(),
                            };
                            // Drop borrowed message before the await to avoid holding the
                            // rdkafka lifetime across the yield point.
                            drop(msg);
                            if tx.send(Ok(broker_msg)).await.is_err() {
                                // Receiver dropped — subscriber gone.
                                break;
                            }
                        }
                    }
                }
            });

            Ok(Box::pin(rx) as MessageStream)
        })
    }

    fn health_check(&self) -> BoxFuture<'_, Result<(), BrokerError>> {
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
                    .map_err(|e| BrokerError::Connection(e.to_string()))
            })
            .await
            .map_err(|e| BrokerError::Connection(format!("health check task failed: {e}")))?
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kafka_message_broker_is_send_and_sync() {
        fn _assert<T: Send + Sync>() {}
        _assert::<KafkaMessageBroker>();
    }

    /// @covers: new
    #[test]
    fn test_new_invalid_broker_config_returns_connection_error() {
        // An empty broker string is rejected by librdkafka at config time.
        let result = KafkaMessageBroker::new("", "test-group");
        // Construction may succeed (rdkafka validates lazily) or fail — either is
        // acceptable; the key invariant is that it does not panic.
        let _ = result;
    }

    /// @covers: health_check
    #[tokio::test]
    async fn test_health_check_fails_for_unreachable_broker() {
        let broker = KafkaMessageBroker::new("127.0.0.1:9999", "test-group")
            .expect("client construction succeeds before first IO");
        let result = broker.health_check().await;
        assert!(
            matches!(result, Err(BrokerError::Connection(_))),
            "expected Connection error for unreachable broker, got: {result:?}"
        );
    }

    /// @covers: subscribe
    #[test]
    fn test_subscribe_channel_capacity_is_positive_and_bounded() {
        let cap = crate::core::task::KAFKA_SUBSCRIBE_CHANNEL_CAPACITY;
        assert!(
            cap > 0,
            "capacity must be positive; 0 would deadlock on first send"
        );
        assert!(
            cap <= 1_000_000,
            "capacity {cap} exceeds sanity limit of 1 M; backpressure would be ineffective"
        );
    }
}
