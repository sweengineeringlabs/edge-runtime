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

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_task_new_creates_with_empty_headers() {
        let t = Task::new(b"payload".as_ref());
        assert_eq!(t.payload.as_ref(), b"payload");
        assert!(t.headers.is_empty());
    }

    /// @covers: with_headers
    #[test]
    fn test_task_with_headers_stores_provided_headers() {
        let mut h = HashMap::new();
        h.insert("correlation-id".into(), "abc123".into());
        let t = Task::with_headers(b"data".as_ref(), h);
        assert_eq!(
            t.headers.get("correlation-id").map(String::as_str),
            Some("abc123")
        );
    }

    /// @covers: with_id
    #[test]
    fn test_task_with_id_uses_provided_id() {
        let id = TaskId::new();
        let t = Task::with_id(id, b"payload".as_ref());
        assert_eq!(t.id, id);
    }

    #[test]
    fn test_task_clone_produces_independent_copy() {
        let t = Task::new(b"data".as_ref());
        let id = t.id;
        let t2 = t.clone();
        assert_eq!(t2.id, id);
        assert_eq!(t2.payload, Bytes::from_static(b"data"));
    }
}
