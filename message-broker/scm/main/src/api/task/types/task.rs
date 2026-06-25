//! [`Task`] — the unit of exchange for [`crate::TaskQueue`].

use std::collections::HashMap;

use bytes::Bytes;

use crate::api::task::types::task_id::TaskId;

/// A task enqueued for exactly one competing consumer to process.
///
/// Tasks carry a payload with optional metadata headers.  Consumers receive
/// tasks via [`crate::TaskQueue::dequeue`] and must acknowledge or reject them
/// via the [`crate::TaskHandle`] returned.
#[derive(Debug, Clone)]
pub struct Task {
    /// Unique task identifier.
    pub id: TaskId,
    /// Raw bytes payload.
    pub payload: Bytes,
    /// Optional key-value metadata headers.
    pub headers: HashMap<String, String>,
}
