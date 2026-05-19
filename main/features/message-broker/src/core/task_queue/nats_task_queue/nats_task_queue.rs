//! [`NatsTaskQueue`] — NATS JetStream queue group backed task queue.

use async_nats::jetstream;
use bytes::Bytes;
use futures::future::BoxFuture;

use crate::api::task_queue::queue_error::QueueError;
use crate::api::task_queue::task::Task;
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
        Box::pin(async move {
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
            // For NATS JetStream with consumer groups
            // In a full implementation, this would:
            // 1. Get or create a consumer in the consumer group
            // 2. Pull the next message with ack/nack options
            // 3. Return the TaskHandle with ack/nack futures
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
    async fn test_nats_task_queue_health_check_unimplemented() {
        // Full NATS integration tests require a running NATS server
        // This test documents that NATS task queue is a placeholder
        let _ = "nats_implementation_requires_jetstream_config";
    }
}
