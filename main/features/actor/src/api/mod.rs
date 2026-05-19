//! Public API: traits and types exposed to consumers.
//!
//! All actor definitions (Actor trait, ActorHandle, ActorContext, MailboxError, StopHandle)
//! live here. SPI implementations remain private and are never re-exported.

pub mod actor;
pub mod traits;

pub use actor::{Actor, ActorContext, ActorHandle, MailboxError, StopHandle};
