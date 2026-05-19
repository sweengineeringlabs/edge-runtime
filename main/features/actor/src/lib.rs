#![warn(missing_docs)]
#![deny(unsafe_code)]

//! Actor runtime — encapsulated state machine with tell/ask semantics.
//!
//! Actors are stateful message processors that own their internal state and process
//! messages sequentially. This crate provides:
//!
//! - **API layer**: [`Actor`] trait, [`ActorHandle`], [`ActorContext`], [`MailboxError`], [`StopHandle`]
//! - **SAF layer**: [`spawn_actor`], [`spawn_actor_with_stop`] — opaque factory functions

pub use crate::api::{Actor, ActorContext, ActorHandle, MailboxError, StopHandle};

#[cfg(feature = "tokio-rt")]
pub use crate::saf::{spawn_actor, spawn_actor_with_stop};

mod api;
mod core;
mod saf;
mod spi;
