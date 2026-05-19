#![warn(missing_docs)]
#![deny(unsafe_code)]

//! Actor runtime — encapsulated state machine with tell/ask semantics.
//!
//! Actors are stateful message processors that own their internal state and process
//! messages sequentially. This crate provides:
//!
//! - **API layer**: [`Actor`] trait, [`ActorHandle`], [`ActorContext`], [`MailboxError`], [`StopHandle`]
//! - **Core layer**: [`crate::core::actor::TokioActor`] — tokio-backed concurrent message loop
//! - **SAF layer**: [`spawn_actor`], [`spawn_actor_with_stop`] — opaque factory functions

pub use crate::gateway::*;

mod api;
mod core;
mod gateway;
mod saf;
