//! [`TaskQueue`] — competing-consumer work queue contract.

use futures::future::BoxFuture;

use crate::api::task::errors::queue_error::QueueError;
use crate::api::task::types::task::task_handle::TaskHandle;
use crate::api::task::types::task::Task;

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
