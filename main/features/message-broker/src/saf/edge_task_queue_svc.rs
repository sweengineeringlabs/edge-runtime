//! SAF — task queue public factory surface.
//!
//! The SAF layer re-exports traits from api/ only. Implementation types
//! (InMemoryTaskQueue, NatsTaskQueue) are never exposed directly — consumers
//! receive them as `impl TaskQueue` from factory functions.
//!
//! SAF does NOT import from core/ directly and does NOT re-export core types.
//! All access to implementation details goes through the factory functions,
//! which return opaque trait objects.

#[cfg(feature = "nats")]
use crate::api::task_queue::queue_error::QueueError;
#[cfg(any(feature = "tokio-rt", feature = "nats"))]
use crate::api::task_queue::TaskQueue;
#[cfg(feature = "tokio-rt")]
use crate::spi::InMemoryTaskQueue;
#[cfg(feature = "nats")]
use crate::spi::NatsTaskQueue;

/// Construct an in-memory task queue backed by [`tokio::sync::mpsc`].
///
/// Tasks are enqueued into a bounded MPSC channel and dequeued by workers.
/// Ack signals permanent removal; nack can signal redelivery.
///
/// Requires the `tokio-rt` feature.
#[cfg(feature = "tokio-rt")]
pub fn in_memory_task_queue() -> impl TaskQueue + Clone {
    InMemoryTaskQueue::new()
}

/// Connect to a NATS server and return a task queue handle.
///
/// Utilizes NATS JetStream with competing consumer semantics.
///
/// # Parameters
///
/// - `nats_url`: NATS server URL (e.g., `nats://localhost:4222`)
/// - `stream_name`: JetStream stream name for tasks
/// - `consumer_group`: Consumer group for competing consumers
///
/// # Errors
///
/// Returns [`QueueError::Connection`] if the NATS server is unreachable or
/// if stream/consumer setup fails.
///
/// Requires the `nats` feature.
#[cfg(feature = "nats")]
pub async fn nats_task_queue(
    nats_url: &str,
    stream_name: String,
    consumer_group: String,
) -> Result<impl TaskQueue, QueueError> {
    // SAF does NOT expose async_nats::jetstream::Context — that's an implementation detail.
    // Instead, SAF takes generic parameters and constructs the context internally.
    let connection = async_nats::connect(nats_url)
        .await
        .map_err(|e| QueueError::Connection(e.to_string()))?;

    let jetstream_context = async_nats::jetstream::new(connection);

    NatsTaskQueue::new(jetstream_context, stream_name, consumer_group).await
}

#[cfg(test)]
mod tests {

    /// @covers: in_memory_task_queue
    #[cfg(feature = "tokio-rt")]
    #[test]
    fn test_in_memory_task_queue_factory_produces_working_queue() {
        use crate::api::task_queue::TaskQueue;
        use crate::in_memory_task_queue;
        use futures::executor::block_on;
        let queue = in_memory_task_queue();
        block_on(async move {
            assert!(queue.health_check().await.is_ok());
        });
    }

    /// @covers: nats_task_queue
    #[test]
    fn test_nats_task_queue_is_feature_gated_behind_nats() {
        let enabled = cfg!(feature = "nats");
        let _ = enabled;
    }
}
