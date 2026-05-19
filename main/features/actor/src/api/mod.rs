//! Public API: traits and types exposed to consumers.
//!
//! All actor definitions (Actor trait, ActorHandle, ActorContext, MailboxError, StopHandle)
//! live here. SPI implementations remain private and are never re-exported.

pub mod actor_context;
pub mod actor_handle;
pub mod actor_mailbox;
pub mod actor_stop_handle;
pub mod mailbox_error;
pub mod traits;

pub use actor_context::ActorContext;
pub use actor_handle::ActorHandle;
pub use actor_mailbox::Actor;
pub use actor_stop_handle::StopHandle;
pub use mailbox_error::MailboxError;
