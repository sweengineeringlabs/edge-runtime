//! SAF — actor public factory surface.
//!
//! Single entry point: edge_runtime_actor_svc.rs concentrates all public factories.
//! Re-exports traits from api/ and factories from svc only.

mod edge_runtime_actor_svc;

pub use crate::api::{Actor, ActorContext, ActorHandle, MailboxError, StopHandle};

#[cfg(feature = "tokio-rt")]
pub use edge_runtime_actor_svc::{spawn_actor, spawn_actor_with_stop};
