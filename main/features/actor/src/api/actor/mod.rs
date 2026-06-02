//! Actor traits.

/// Actor trait — message-handling state machine.
#[allow(clippy::module_inception)]
pub mod actor;
/// Actor handle for communication.
pub mod actor_handle;
/// Stop signal handle.
pub mod stop_handle;

pub use actor::Actor;
pub use actor_handle::ActorHandle;
pub use stop_handle::StopHandle;
