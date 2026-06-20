//! [`TaskHandle`] — returned by dequeue, must be acked or nacked.

use std::collections::HashMap;

use bytes::Bytes;
use futures::future::BoxFuture;

use crate::api::task::errors::queue_error::QueueError;
use crate::api::task::types::task_id::TaskId;

/// Returned by [`crate::TaskQueue::dequeue`]. Consumer MUST call `ack` or `nack`.
///
/// If neither is called before the visibility timeout, the message reappears
/// in the queue for redelivery.
pub struct TaskHandle {
    /// The ID of the dequeued task.
    pub task_id: TaskId,
    /// Raw bytes payload of the dequeued task.
    pub payload: Bytes,
    /// Optional key-value metadata headers of the dequeued task.
    pub headers: HashMap<String, String>,
    /// Acknowledge the task: remove it from the queue permanently.
    pub ack: BoxFuture<'static, Result<(), QueueError>>,
    /// Reject the task: return it to the queue for redelivery.
    pub nack: BoxFuture<'static, Result<(), QueueError>>,
}

impl TaskHandle {
    /// Create a [`TaskHandle`] from the dequeued task's primitives and its ack/nack futures.
    pub fn new(
        task_id: TaskId,
        payload: Bytes,
        headers: HashMap<String, String>,
        ack: BoxFuture<'static, Result<(), QueueError>>,
        nack: BoxFuture<'static, Result<(), QueueError>>,
    ) -> Self {
        Self {
            task_id,
            payload,
            headers,
            ack,
            nack,
        }
    }
}
