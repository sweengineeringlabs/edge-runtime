//! Mailbox errors from actor operations.

use std::fmt;

/// Errors that can occur during actor message operations.
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
