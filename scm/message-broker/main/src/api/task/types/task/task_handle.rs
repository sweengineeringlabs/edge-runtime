//! [`TaskHandle`] — returned by dequeue, must be acked or nacked.

use futures::future::BoxFuture;

use crate::api::task::errors::queue_error::QueueError;
use crate::api::task::types::task::task_id::TaskId;
use crate::api::task::types::task::Task;

/// Returned by [`crate::TaskQueue::dequeue`]. Consumer MUST call `ack` or `nack`.
///
/// If neither is called before the visibility timeout, the message reappears
/// in the queue for redelivery.
pub struct TaskHandle {
    /// The ID of the dequeued task (mirrors `task.id` for convenience).
    pub task_id: TaskId,
    /// The dequeued task, including its payload and headers.
    pub task: Task,
    /// Acknowledge the task: remove it from the queue permanently.
    pub ack: BoxFuture<'static, Result<(), QueueError>>,
    /// Reject the task: return it to the queue for redelivery.
    pub nack: BoxFuture<'static, Result<(), QueueError>>,
}

impl TaskHandle {
    /// Create a [`TaskHandle`] from a dequeued [`Task`] and its ack/nack futures.
    pub fn new(
        task: Task,
        ack: BoxFuture<'static, Result<(), QueueError>>,
        nack: BoxFuture<'static, Result<(), QueueError>>,
    ) -> Self {
        let task_id = task.id;
        Self {
            task_id,
            task,
            ack,
            nack,
        }
    }
}
