//! [`TaskHandle`] — returned by dequeue, must be acked or nacked.

use futures::future::BoxFuture;

use crate::api::task::queue::queue_error::QueueError;
use crate::api::task::queue::types::task::task_id::TaskId;

/// Returned by [`crate::TaskQueue::dequeue`]. Consumer MUST call `ack` or `nack`.
///
/// If neither is called before the visibility timeout, the message reappears
/// in the queue for redelivery.
pub struct TaskHandle {
    /// The ID of this task for reference.
    pub task_id: TaskId,
    /// Acknowledge the task: remove it from the queue permanently.
    pub ack: BoxFuture<'static, Result<(), QueueError>>,
    /// Reject the task: return it to the queue for redelivery.
    pub nack: BoxFuture<'static, Result<(), QueueError>>,
}

impl TaskHandle {
    /// Create a TaskHandle with ack and nack futures.
    pub fn new(
        task_id: TaskId,
        ack: BoxFuture<'static, Result<(), QueueError>>,
        nack: BoxFuture<'static, Result<(), QueueError>>,
    ) -> Self {
        Self { task_id, ack, nack }
    }
}
