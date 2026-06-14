//! SAF — runtime service surface.
mod runtime_manager_svc;
pub mod runtime_svc;
pub use runtime_manager_svc::RUNTIME_MANAGER_SVC;
pub use runtime_svc::*;
