//! Public value types for the actor crate.

pub mod actor;
pub mod application_config_builder;
pub mod message;

#[cfg(feature = "tokio-rt")]
#[expect(
    unused_imports,
    reason = "SEA api/ anchor — ActorSpawnHandle exported for consumers, not used internally"
)]
pub use actor::ActorSpawnHandle;
pub use actor::{ActorContext, ActorRuntime};
#[expect(
    unused_imports,
    reason = "SEA api/ anchor — ApplicationConfigBuilder exported for consumers, not used internally"
)]
pub use application_config_builder::ApplicationConfigBuilder;
pub use message::Message;
