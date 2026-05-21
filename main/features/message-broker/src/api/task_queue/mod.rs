//! Task queue API — traits, value types, and error definitions.

use futures::future::BoxFuture;

pub mod queue_error;
pub mod task;
pub mod task_handle;

pub use queue_error::QueueError;
pub use task::{Task, TaskId};
pub use task_handle::TaskHandle;

/// Competing-consumer work queue contract.
///
/// A task queue delivers each message to exactly one competing consumer, which
/// must acknowledge or reject it. Unlike [`crate::MessageBroker`], which fans
/// out every message to all subscribers, a task queue distributes work.
///
/// # Enqueue semantics
///
/// Enqueuing a task succeeds immediately. The task becomes available for
/// dequeue by any consumer.
///
/// # Dequeue semantics
///
/// Each call to [`dequeue`] returns an independent [`TaskHandle`] for exactly
/// one task. If no tasks are available, implementations may block, return
/// `None`, or return an error. See the specific implementation for details.
///
/// A consumer MUST call either `ack()` or `nack()` on the handle:
/// - **ack()** — task is removed from the queue permanently
/// - **nack()** — task is returned to the queue for redelivery after a
///   visibility timeout
///
/// If neither is called, the task reappears after the visibility timeout.
///
/// [`dequeue`]: TaskQueue::dequeue
pub trait TaskQueue: Send + Sync {
    /// Enqueue a task for exactly one consumer to process.
    fn enqueue(&self, task: Task) -> BoxFuture<'_, Result<(), QueueError>>;

    /// Dequeue the next available task. Returns `None` if the queue is empty.
    fn dequeue(&self) -> BoxFuture<'_, Result<Option<TaskHandle>, QueueError>>;

    /// Probe queue connectivity. Returns `Ok(())` if the queue is reachable.
    fn health_check(&self) -> BoxFuture<'_, Result<(), QueueError>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_queue_is_object_safe() {
        fn _check(_: &dyn TaskQueue) {}
    }
}
