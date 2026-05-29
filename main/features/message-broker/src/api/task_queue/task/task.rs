//! [`Task`] — the unit of exchange for [`crate::TaskQueue`].

use std::collections::HashMap;

use bytes::Bytes;

use super::task_id::TaskId;

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

impl Task {
    /// Construct a task from raw bytes with no headers.
    pub fn new(payload: impl Into<Bytes>) -> Self {
        Self {
            id: TaskId::new(),
            payload: payload.into(),
            headers: HashMap::new(),
        }
    }

    /// Construct a task with headers.
    pub fn with_headers(payload: impl Into<Bytes>, headers: HashMap<String, String>) -> Self {
        Self {
            id: TaskId::new(),
            payload: payload.into(),
            headers,
        }
    }

    /// Construct a task with a specific ID.
    pub fn with_id(id: TaskId, payload: impl Into<Bytes>) -> Self {
        Self {
            id,
            payload: payload.into(),
            headers: HashMap::new(),
        }
    }
}
