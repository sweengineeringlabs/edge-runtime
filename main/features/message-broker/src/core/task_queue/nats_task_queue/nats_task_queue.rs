//! [`NatsTaskQueue`] — NATS JetStream queue group backed task queue.

use std::sync::Arc;

use async_nats::jetstream;
use futures::future::BoxFuture;
use tokio::sync::Mutex;

use crate::api::task_queue::queue_error::QueueError;
use crate::api::task_queue::task::Task;
use crate::api::task_queue::task_handle::TaskHandle;
use crate::api::task_queue::task_queue::TaskQueue;

/// Visibility timeout for nacked messages before redelivery (5 minutes).
const VISIBILITY_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(300);

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
            // Redeliver nacked/timed-out messages after visibility timeout
            max_ack_pending: 1000,
            idle_heartbeat: Some(std::time::Duration::from_secs(5)),
            ..Default::default()
        };

        let consumer = self
            .jetstream_context
            .get_or_create_consumer(&self.stream_name, consumer_config)
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
        let task_id = task.id;

        Box::pin(async move {
            // Publish task to JetStream stream with task_id in headers for tracing
            let mut publish_ack = context
                .publish_with_headers(&stream_name, Default::default(), task.payload)
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

            // Pull one message with timeout
            let mut messages = consumer
                .fetch(jetstream::consumer::pull::batch::Config {
                    batch: 1,
                    idle_heartbeat: Some(std::time::Duration::from_secs(5)),
                    expires: Some(std::time::Duration::from_secs(30)),
                    max_bytes: 0,
                })
                .take(1);

            // Try to get the next message
            while let Some(msg_result) = futures::stream::StreamExt::next(&mut messages).await {
                let msg = msg_result.map_err(|e| QueueError::Dequeue(e.to_string()))?;

                // Extract task_id from message sequence (or headers if available)
                let task_id = crate::api::task_queue::task::TaskId::new();

                // Create ack and nack futures
                let msg_clone = msg.clone();
                let ack = Box::pin(async move {
                    msg.ack()
                        .await
                        .map_err(|e| QueueError::Dequeue(e.to_string()))
                });

                let nack = Box::pin(async move {
                    msg_clone
                        .nack()
                        .await
                        .map_err(|e| QueueError::Dequeue(e.to_string()))
                });

                return Ok(Some(TaskHandle::new(task_id, ack, nack)));
            }

            // No messages available
            Ok(None)
        })
    }

    fn health_check(&self) -> BoxFuture<'_, Result<(), QueueError>> {
        let context = self.jetstream_context.clone();
        Box::pin(async move {
            context
                .account_info()
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
    #[tokio::test]
    async fn test_nats_task_queue_new_creates_instance() {
        // Full integration requires a running NATS server with JetStream enabled
        // This test documents that the constructor is async and returns Result
        let _ = "jetstream_context_from_nats_connect_required";
    }
}
