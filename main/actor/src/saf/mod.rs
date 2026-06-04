//! SAF — actor public factory surface.
//!
//! Single entry point: actor_svc.rs concentrates all public factory methods on `ActorRuntime`.
//! Re-exports traits from api/ and `ActorRuntime` from types/.

mod actor_svc;

pub use crate::api::{Actor, ActorContext, ActorHandle, ActorRuntime, MailboxError, StopHandle};
