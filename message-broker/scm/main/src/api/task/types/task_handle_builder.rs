//! [`TaskHandleBuilder`] — fluent constructor for [`TaskHandle`].

use std::collections::HashMap;

use bytes::Bytes;
use futures::future::BoxFuture;

use crate::api::task::errors::queue_error::QueueError;
use crate::api::task::types::task_id::TaskId;

/// Fluent builder for [`TaskHandle`].
///
/// All required fields (`task_id`, `payload`, `ack`, `nack`) are supplied at
/// construction time; optional fields (`headers`) use fluent setters.
/// Call [`build`](TaskHandleBuilder::build) to produce the [`TaskHandle`].
pub struct TaskHandleBuilder {
    pub(crate) task_id: TaskId,
    pub(crate) payload: Bytes,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) ack: BoxFuture<'static, Result<(), QueueError>>,
    pub(crate) nack: BoxFuture<'static, Result<(), QueueError>>,
}
