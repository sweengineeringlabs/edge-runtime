#![warn(missing_docs)]
#![deny(unsafe_code)]

//! Actor runtime — encapsulated state machine with tell/ask semantics.
//!
//! Actors are stateful message processors that own their internal state and process
//! messages sequentially. Single entry point: [`crate::gateway`] (via `ActorRuntime`).

pub use crate::gateway::*;

mod api;
mod core;
mod gateway;
mod saf;
mod spi;
