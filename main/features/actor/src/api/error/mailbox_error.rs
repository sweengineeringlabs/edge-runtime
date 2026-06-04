//! Mailbox errors from actor operations.

use std::fmt;

/// Errors that can occur during actor message operations.
///
/// Returned by [`ActorHandle::tell`](crate::ActorHandle::tell). All variants
/// indicate that the message was not delivered. `Full` is transient (retry
/// after back-off); `Closed` and `ActorStopped` are terminal.
///
/// # Examples
///
/// ```rust
/// use swe_edge_runtime_actor::MailboxError;
///
/// let err = MailboxError::Full;
/// assert!(err.to_string().contains("capacity"));
///
/// let err = MailboxError::Closed;
/// assert!(err.to_string().contains("dropped"));
///
/// // Classify for retry logic.
/// fn is_retryable(e: &MailboxError) -> bool {
///     matches!(e, MailboxError::Full)
/// }
/// assert!(is_retryable(&MailboxError::Full));
/// assert!(!is_retryable(&MailboxError::ActorStopped));
/// ```
#[derive(Debug, Clone)]
pub enum MailboxError {
    /// Mailbox capacity exceeded (bounded queue full).
    Full,
    /// Actor receiver dropped (actor stopped).
    Closed,
    /// Reply channel dropped before response received.
    ReplyDropped,
    /// Actor panicked or stopped unexpectedly.
    ActorStopped,
}

impl fmt::Display for MailboxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Full => write!(f, "mailbox capacity exceeded"),
            Self::Closed => write!(f, "actor receiver dropped"),
            Self::ReplyDropped => write!(f, "reply channel dropped"),
            Self::ActorStopped => write!(f, "actor stopped or panicked"),
        }
    }
}

impl std::error::Error for MailboxError {}
