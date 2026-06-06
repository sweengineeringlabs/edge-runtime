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
