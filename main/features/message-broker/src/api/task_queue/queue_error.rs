//! [`QueueError`] — error variants for task queue operations.

/// Errors returned by [`crate::TaskQueue`] operations.
#[derive(Debug, thiserror::Error)]
pub enum QueueError {
    /// An enqueue operation failed.
    #[error("enqueue failed: {0}")]
    Enqueue(String),

    /// A dequeue operation failed.
    #[error("dequeue failed: {0}")]
    Dequeue(String),

    /// The mailbox is full (bounded channel capacity exceeded).
    #[error("queue full")]
    Full,

    /// The queue has shut down and is no longer accepting messages.
    #[error("queue closed")]
    Closed,

    /// The ask reply sender was dropped before sending a response.
    #[error("reply sender dropped")]
    ReplyDropped,

    /// The queue connection could not be established or was lost.
    #[error("queue connection failed: {0}")]
    Connection(String),

    /// The queue is temporarily unavailable.
    #[error("queue unavailable: {0}")]
    Unavailable(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enqueue_error_includes_reason() {
        let e = QueueError::Enqueue("serialization failed".into());
        assert!(e.to_string().contains("serialization failed"));
    }

    #[test]
    fn test_dequeue_error_includes_reason() {
        let e = QueueError::Dequeue("connection lost".into());
        assert!(e.to_string().contains("connection lost"));
    }

    #[test]
    fn test_full_error_displays() {
        let e = QueueError::Full;
        assert_eq!(e.to_string(), "queue full");
    }

    #[test]
    fn test_closed_error_displays() {
        let e = QueueError::Closed;
        assert_eq!(e.to_string(), "queue closed");
    }
}
