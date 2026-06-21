//! SAF — lifecycle service surface.
pub mod lifecycle_monitor;
mod lifecycle_observer_svc;
pub use lifecycle_monitor::*;
pub use lifecycle_observer_svc::LIFECYCLE_OBSERVER_SVC;
