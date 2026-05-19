//! Public API: traits and types exposed to consumers.
//!
//! All actor definitions (Actor trait, ActorHandle, ActorContext, MailboxError, StopHandle)
//! live here. Core implementations (TokioActor) remain in core/ and are never re-exported.

pub mod actor;
pub mod context;
pub mod error;
pub mod handle;
pub mod stop_handle;

pub use actor::Actor;
pub use context::ActorContext;
pub use error::MailboxError;
pub use handle::ActorHandle;
pub use stop_handle::StopHandle;
