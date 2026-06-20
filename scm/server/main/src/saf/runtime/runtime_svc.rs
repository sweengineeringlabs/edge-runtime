//! SAF — runtime type re-exports.

pub use crate::api::ComponentHealth;
pub use crate::api::ConfigError;
pub use crate::api::ConfigLoader;
pub use crate::api::Egress;
pub use crate::api::Ingress;
pub use crate::api::RuntimeConfig;
pub use crate::api::RuntimeHealth;
pub use crate::api::RuntimeManager;
pub use crate::api::RuntimeStatus;
pub use crate::api::ServiceRegistry;
pub use crate::api::{Runtime, RuntimeBuilder};
pub use crate::api::{RuntimeError, RuntimeResult};
pub use crate::api::{ServerConfigLoader, ServerMonitor};

// ── Load monitoring / auto-scaling ────────────────────────────────────────────
pub use crate::api::MetricsConfig;
pub use crate::api::RingBuffer;
pub use crate::api::{
    AutoscalePolicy, ScalingDecision, ScalingPolicy, SharedCounters, ThresholdPolicy,
    TrafficCounters,
};

// ── Observability ─────────────────────────────────────────────────────────────
#[cfg(feature = "observability")]
pub use crate::api::TracingInitializer;

/// Semantic version of the runtime API surface published by this crate.
pub const RUNTIME_API_VERSION: &str = "0.3";
