//! SAF — runtime type re-exports.

pub use crate::api::config::traits::config_loader::ConfigLoader;
pub use crate::api::config::ConfigError;
pub use crate::api::config::RuntimeConfig;
pub use crate::api::egress::Egress;
pub use crate::api::ingress::Ingress;
pub use crate::api::runtime::traits::runtime_manager::RuntimeManager;
pub use crate::api::runtime::types::component_health::ComponentHealth;
pub use crate::api::runtime::types::runtime_health::RuntimeHealth;
pub use crate::api::runtime::types::runtime_status::RuntimeStatus;
pub use crate::api::runtime::ServiceRegistry;
pub use crate::api::runtime::{Runtime, RuntimeBuilder};
pub use crate::api::runtime::{RuntimeError, RuntimeResult};
pub use crate::api::runtime::{ServerConfigLoader, ServerMonitor};

// ── Load monitoring / auto-scaling ────────────────────────────────────────────
pub use crate::api::metrics::MetricsConfig;
pub use crate::api::monitor::types::ring_buffer::RingBuffer;
pub use crate::api::monitor::{
    AutoscalePolicy, ScalingDecision, ScalingPolicy, SharedCounters, ThresholdPolicy,
    TrafficCounters,
};

// ── Observability ─────────────────────────────────────────────────────────────
#[cfg(feature = "observability")]
pub use crate::api::runtime::TracingInitializer;

/// Semantic version of the runtime API surface published by this crate.
pub const RUNTIME_API_VERSION: &str = "0.3";
