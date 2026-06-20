//! Lifecycle monitor interface — canonical re-export from `edge_proxy`.

pub use edge_proxy::LifecycleMonitor;

/// Grace period in seconds the lifecycle monitor waits before reporting unhealthy.
pub const HEALTH_CHECK_GRACE_PERIOD_SECS: u64 = 5;
