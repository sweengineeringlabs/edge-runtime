//! Output gateway — outbound service contracts.

#[allow(unused_imports)]
pub use crate::api::{ActorHandle, StopHandle};

#[cfg(feature = "tokio-rt")]
#[allow(unused_imports)]
pub use crate::saf::{spawn_actor, spawn_actor_with_stop};
