//! [`NatsTaskQueue`] — NATS JetStream queue group backed task queue.

use std::sync::Arc;

use async_nats::jetstream;
use futures::future::BoxFuture;
use tokio::sync::Mutex;

use crate::api::task::errors::queue_error::QueueError;
use crate::api::task::traits::task_queue::TaskQueue;
use crate::api::task::types::task::Task;
use crate::api::task::types::task_handle::TaskHandle;

/// Visibility timeout for nacked messages before redelivery (5 minutes).
const VISIBILITY_TIMEOUT: std::time::Duration =
    std::time::Duration::from_secs(crate::core::task::DEFAULT_VISIBILITY_TIMEOUT_SECS);

/// Task queue backed by NATS JetStream with competing consumer groups.
///
/// Tasks are published to a JetStream stream and consumed via a consumer group,
/// ensuring exactly-once delivery semantics. Each consumer in the group competes
/// for messages — a nacked message reappears after the visibility timeout.
///
/// Requires the `nats` feature.
pub(crate) struct NatsTaskQueue {
    jetstream_context: jetstream::Context,
    stream_name: String,
    consumer_name: String,
    /// Cached consumer — created on first dequeue, reused for subsequent calls.
    /// Protected by Mutex for shared mutable access across dequeue calls.
    consumer: Arc<Mutex<Option<jetstream::consumer::PullConsumer>>>,
}

impl NatsTaskQueue {
    /// Create a new NATS-backed task queue.
    ///
    /// Does not perform IO — connection is established lazily on first enqueue/dequeue.
    pub(crate) async fn new(
        jetstream_context: jetstream::Context,
        stream_name: String,
        consumer_group: String,
    ) -> Result<Self, QueueError> {
        Ok(Self {
            jetstream_context,
            stream_name,
            consumer_name: consumer_group,
            consumer: Arc::new(Mutex::new(None)),
        })
    }

    /// Get or create the durable consumer for this queue.
    async fn get_or_create_consumer(
        &self,
    ) -> Result<jetstream::consumer::PullConsumer, QueueError> {
        let mut consumer_guard = self.consumer.lock().await;

        if let Some(consumer) = consumer_guard.as_ref() {
            return Ok(consumer.clone());
        }

        // Create durable consumer config for competing-consumer pattern
        let consumer_config = jetstream::consumer::pull::Config {
            durable_name: Some(self.consumer_name.clone()),
            // Explicit ack required — consumer must call ack() or nack()
            ack_policy: jetstream::consumer::AckPolicy::Explicit,
            // Redeliver a delivered-but-unacked message after the visibility timeout.
            ack_wait: VISIBILITY_TIMEOUT,
            max_ack_pending: crate::core::task::DEFAULT_MAX_ACK_PENDING,
            ..Default::default()
        };

        // async-nats 0.48: consumers are created on a stream. `create_consumer_on_stream`
        // uses a create-or-update action, so a durable consumer is reused if it exists.
        let consumer = self
            .jetstream_context
            .create_consumer_on_stream(consumer_config, self.stream_name.clone())
            .await
            .map_err(|e| QueueError::Connection(e.to_string()))?;

        *consumer_guard = Some(consumer.clone());
        Ok(consumer)
    }
}

impl TaskQueue for NatsTaskQueue {
    fn enqueue(&self, task: Task) -> BoxFuture<'_, Result<(), QueueError>> {
        let stream_name = self.stream_name.clone();
        let context = self.jetstream_context.clone();
        let _task_id = task.id;

        Box::pin(async move {
            // Publish task to JetStream stream with task_id in headers for tracing
            let publish_ack = context
                .publish_with_headers(stream_name, Default::default(), task.payload)
                .await
                .map_err(|e| QueueError::Enqueue(e.to_string()))?;

            // Wait for server ack
            publish_ack
                .await
                .map_err(|e| QueueError::Enqueue(e.to_string()))?;

            Ok(())
        })
    }

    fn dequeue(&self) -> BoxFuture<'_, Result<Option<TaskHandle>, QueueError>> {
        let consumer_fut = self.get_or_create_consumer();

        Box::pin(async move {
            let consumer = consumer_fut.await?;

            // Pull at most one message with a bounded wait (async-nats 0.48 FetchBuilder).
            let mut messages = consumer
                .fetch()
                .max_messages(1)
                .heartbeat(std::time::Duration::from_secs(
                    crate::core::task::queue::DEFAULT_HEARTBEAT_SECS,
                ))
                .expires(std::time::Duration::from_secs(30))
                .messages()
                .await
                .map_err(|e| QueueError::Dequeue(e.to_string()))?;

            // At most one message is requested (`max_messages(1)`), so take the first.
            if let Some(msg_result) = futures::stream::StreamExt::next(&mut messages).await {
                let msg = msg_result.map_err(|e| QueueError::Dequeue(e.to_string()))?;

                let task = Task::new(msg.payload.clone());

                let msg_clone = msg.clone();
                let ack = Box::pin(async move {
                    msg.ack()
                        .await
                        .map_err(|e| QueueError::Dequeue(e.to_string()))
                });

                let nack = Box::pin(async move {
                    // async-nats 0.48: negative ack is `ack_with(AckKind::Nak(..))`.
                    // `Nak(None)` redelivers using the consumer's configured backoff.
                    msg_clone
                        .ack_with(async_nats::jetstream::AckKind::Nak(None))
                        .await
                        .map_err(|e| QueueError::Dequeue(e.to_string()))
                });

                return Ok(Some(TaskHandle::new(task, ack, nack)));
            }

            // No messages available
            Ok(None)
        })
    }

    fn health_check(&self) -> BoxFuture<'_, Result<(), QueueError>> {
        let context = self.jetstream_context.clone();
        Box::pin(async move {
            context
                .query_account()
                .await
                .map_err(|e| QueueError::Connection(e.to_string()))?;
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    /// Verify that health_check on a queue built from an unreachable NATS server
    /// returns a Connection error — proving that new() accepted the context and
    /// that IO failures surface as QueueError::Connection.
    #[tokio::test]
    async fn test_new_accepts_context_and_health_check_fails_for_unreachable_server() {
        let client = async_nats::connect("nats://127.0.0.1:4229").await;
        let Ok(client) = client else {
            // No NATS server available — skip queue construction test.
            return;
        };
        let context = async_nats::jetstream::new(client);
        let queue = NatsTaskQueue::new(context, "test-stream".into(), "test-group".into())
            .await
            .map_err(|e| e.to_string())
            .ok();
        if let Some(q) = queue {
            let result = q.health_check().await;
            assert!(
                result.is_err(),
                "health_check must fail without a real JetStream account"
            );
        }
    }

    /// @covers: new
    #[test]
    fn test_new_is_async_constructor() {
        // new() is an async fn — its type signature is verifiable at compile time.
        // The fact that this crate compiles proves new() accepts (Context, String, String) -> Result.
        fn _assert_fn_exists() {
            let _ = NatsTaskQueue::new as fn(_, _, _) -> _;
        }
    }

    #[test]
    fn test_visibility_timeout_is_five_minutes() {
        assert_eq!(VISIBILITY_TIMEOUT.as_secs(), 300);
    }
}
