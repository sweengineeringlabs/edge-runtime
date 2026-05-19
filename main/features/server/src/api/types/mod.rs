//! Value types for the daemon runtime.

pub mod runtime;
pub mod runtime_health;

pub use runtime::RuntimeConfig;
pub use runtime::RuntimeStatus;
pub use runtime_health::RuntimeHealth;
