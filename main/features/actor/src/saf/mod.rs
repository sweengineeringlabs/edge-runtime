//! SAF — actor public factory surface.
//!
//! The SAF layer re-exports traits from api/ only. Implementation types
//! (TokioActorHandle, TokioStopHandle) are never exposed directly — consumers
//! receive them as `impl ActorHandle` and `impl StopHandle` from factory functions.
//!
//! SAF does NOT import from core/ directly and does NOT re-export core types.
//! All access to implementation details goes through the factory functions.

#[cfg(feature = "tokio-rt")]
mod edge_runtime_actor_svc;

#[cfg(feature = "tokio-rt")]
pub use edge_runtime_actor_svc::{spawn_actor, spawn_actor_with_stop};
