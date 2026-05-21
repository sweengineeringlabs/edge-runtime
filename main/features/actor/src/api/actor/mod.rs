//! Actor mailbox types and traits.

/// Actor execution context.
pub mod context;
/// Mailbox error types.
pub mod error;
/// Actor handle for communication.
pub mod handle;
/// Actor mailbox implementation.
pub mod mailbox;
/// Stop signal handle.
pub mod stop_handle;

pub use context::ActorContext;
pub use error::MailboxError;
pub use handle::ActorHandle;
pub use mailbox::Actor;
pub use stop_handle::StopHandle;
