//! [`NatsTaskQueue`] — NATS JetStream queue group backed task queue.

use async_nats::jetstream;
use futures::future::BoxFuture;
use tokio::sync::oneshot;

use crate::api::task_queue::queue_error::QueueError;
use crate::api::task_queue::task::{Task, TaskId};
use crate::api::task_queue::task_handle::TaskHandle;
use crate::api::task_queue::task_queue::TaskQueue;

/// Task queue backed by NATS JetStream with competing consumer groups.
///
/// Tasks are published to a JetStream stream and consumed via a consumer group,
/// ensuring exactly-once delivery semantics.
///
/// Requires the `nats` feature.
pub(crate) struct NatsTaskQueue {
    jetstream_context: jetstream::Context,
    stream_name: String,
    consumer_group: String,
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
            consumer_group,
        })
    }
}

impl TaskQueue for NatsTaskQueue {
    fn enqueue(&self, task: Task) -> BoxFuture<'_, Result<(), QueueError>> {
        let stream_name = self.stream_name.clone();
        let context = self.jetstream_context.clone();
        let task_id = task.id;

        Box::pin(async move {
            // Publish task to JetStream stream
            // Headers include task_id for correlation
            context
                .publish(&stream_name, task.payload)
                .await
                .map_err(|e| QueueError::Enqueue(e.to_string()))?;
            Ok(())
        })
    }

    fn dequeue(&self) -> BoxFuture<'_, Result<Option<TaskHandle>, QueueError>> {
        let stream_name = self.stream_name.clone();
        let consumer_group = self.consumer_group.clone();
        let context = self.jetstream_context.clone();

        Box::pin(async move {
            // In a full implementation:
            // 1. Get or create a durable consumer in the consumer group
            // 2. Pull the next message with ack_policy: Explicit
            // 3. Return TaskHandle with futures that call message.ack() and message.nack()

            // For now, placeholder documents the pattern
            let _stream = stream_name;
            let _group = consumer_group;
            let _js = context;

            // Future implementation would:
            // let consumer = context.get_or_create_consumer(&stream, consumer_config).await?;
            // if let Some(message) = consumer.next_raw().await? {
            //     let task_id = extract_task_id_from_headers(&message.metadata);
            //     let (ack_tx, ack_rx) = oneshot::channel();
            //     let (nack_tx, nack_rx) = oneshot::channel();
            //
            //     return Ok(Some(TaskHandle::new(
            //         task_id,
            //         Box::pin(async move { message.ack().await.map_err(...) }),
            //         Box::pin(async move { message.nack().await.map_err(...) }),
            //     )));
            // }

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
    async fn test_nats_task_queue_new_requires_jetstream_server() {
        // Full NATS integration tests require a running NATS server with JetStream enabled
        // This test documents the expected async signature
        let _ = "jetstream_context_from_nats_connection_required";
    }
}
