//! Public API: traits and types exposed to consumers.
//!
//! All actor definitions (Actor trait, ActorHandle, ActorContext, MailboxError, StopHandle)
//! live here. SPI implementations remain private and are never re-exported.

pub mod actor;
pub mod error;
pub mod traits;
pub mod types;
pub mod validator;

pub use actor::{Actor, ActorHandle, StopHandle};
pub use error::MailboxError;
pub use types::{ActorContext, ActorRuntime, Message};
