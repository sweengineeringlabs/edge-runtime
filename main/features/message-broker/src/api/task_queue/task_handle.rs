//! [`TaskHandle`] — returned by dequeue, must be acked or nacked.

use futures::future::BoxFuture;

use crate::api::task_queue::queue_error::QueueError;

/// Returned by [`crate::TaskQueue::dequeue`]. Consumer MUST call `ack` or `nack`.
///
/// If neither is called before the visibility timeout, the message reappears
/// in the queue for redelivery.
pub struct TaskHandle {
    /// The ID of this task for reference.
    pub task_id: super::task::TaskId,
    /// Acknowledge the task: remove it from the queue permanently.
    pub ack: BoxFuture<'static, Result<(), QueueError>>,
    /// Reject the task: return it to the queue for redelivery.
    pub nack: BoxFuture<'static, Result<(), QueueError>>,
}

impl TaskHandle {
    /// Create a TaskHandle with ack and nack futures.
    pub fn new(
        task_id: super::task::TaskId,
        ack: BoxFuture<'static, Result<(), QueueError>>,
        nack: BoxFuture<'static, Result<(), QueueError>>,
    ) -> Self {
        Self { task_id, ack, nack }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_task_handle_new_stores_task_id() {
        let task_id = super::super::task::TaskId::new();
        let ack = Box::pin(async { Ok(()) });
        let nack = Box::pin(async { Ok(()) });
        let handle = TaskHandle::new(task_id, ack, nack);
        assert_eq!(handle.task_id, task_id);
    }
}
