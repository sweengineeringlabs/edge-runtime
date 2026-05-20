//! Actor mailbox types and traits.

pub mod context;
pub mod error;
pub mod handle;
pub mod mailbox;
pub mod stop_handle;

pub use context::ActorContext;
pub use error::MailboxError;
pub use handle::ActorHandle;
pub use mailbox::Actor;
pub use stop_handle::StopHandle;
