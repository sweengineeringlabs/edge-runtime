//! [`TaskHandleBuilder`] — fluent constructor for [`TaskHandle`].

use std::collections::HashMap;

use bytes::Bytes;
use futures::future::BoxFuture;

use crate::api::task::errors::queue_error::QueueError;
use crate::api::task::types::task_handle::TaskHandle;
use crate::api::task::types::task_id::TaskId;

/// Fluent builder for [`TaskHandle`].
///
/// All required fields (`task_id`, `payload`, `ack`, `nack`) are supplied at
/// construction time; optional fields (`headers`) use fluent setters.
/// Call [`build`](TaskHandleBuilder::build) to produce the [`TaskHandle`].
pub struct TaskHandleBuilder {
    task_id: TaskId,
    payload: Bytes,
    headers: HashMap<String, String>,
    ack: BoxFuture<'static, Result<(), QueueError>>,
    nack: BoxFuture<'static, Result<(), QueueError>>,
}

impl TaskHandleBuilder {
    /// Create a builder with all required fields.
    pub fn new(
        task_id: TaskId,
        payload: Bytes,
        ack: BoxFuture<'static, Result<(), QueueError>>,
        nack: BoxFuture<'static, Result<(), QueueError>>,
    ) -> Self {
        Self {
            task_id,
            payload,
            headers: HashMap::new(),
            ack,
            nack,
        }
    }

    /// Set the optional key-value metadata headers for the task handle.
    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = headers;
        self
    }

    /// Consume the builder and produce a [`TaskHandle`].
    pub fn build(self) -> TaskHandle {
        TaskHandle::new(
            self.task_id,
            self.payload,
            self.headers,
            self.ack,
            self.nack,
        )
    }
}
