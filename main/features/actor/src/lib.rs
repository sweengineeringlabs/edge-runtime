#![warn(missing_docs)]
#![deny(unsafe_code)]

//! Actor runtime — encapsulated state machine with tell/ask semantics.
//!
//! Actors are stateful message processors that own their internal state and process
//! messages sequentially. Single entry point: [`crate::saf`] (edge_runtime_actor_svc).

pub use crate::saf::*;

mod api;
mod core;
mod saf;
mod spi;
