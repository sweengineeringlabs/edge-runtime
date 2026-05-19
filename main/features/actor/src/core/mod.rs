//! Core implementations — never re-exported directly.
//!
//! The core/ layer contains implementation types like TokioActor that are internal
//! to this crate. Consumers access them only through SAF factory functions which
//! return opaque `impl ActorHandle` or `impl StopHandle` trait objects.

#[cfg(feature = "tokio-rt")]
pub(crate) mod actor;
