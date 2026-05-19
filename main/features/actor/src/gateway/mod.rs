//! Public re-exports from API and SAF layers.

pub use crate::api::{Actor, ActorContext, ActorHandle, MailboxError, StopHandle};

#[cfg(feature = "tokio-rt")]
pub use crate::saf::{spawn_actor, spawn_actor_with_stop};
