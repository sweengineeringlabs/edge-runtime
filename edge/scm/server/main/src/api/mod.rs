//! API layer — runtime trait contracts and public types, organised by theme.

mod composite;
mod config;
mod egress;
mod ingress;
mod json;
mod metrics;
mod monitor;
mod runner;
mod runtime;
mod validator;

// ── composite ────────────────────────────────────────────────────────────────
pub use composite::{CompositeGrpcIngress, CompositeIngress};

// ── config ───────────────────────────────────────────────────────────────────
pub use config::{
    ApplicationConfigLoader, ConfigError, ConfigLoader, ObservabilityConfig, RuntimeConfig,
    TracingConfig,
};
// ConfigOverride is pub(crate) — accessible within the crate via crate::api::config::ConfigOverride
pub(crate) use config::ConfigOverride;

// ── egress ───────────────────────────────────────────────────────────────────
pub use egress::Egress;

// ── ingress ──────────────────────────────────────────────────────────────────
pub use ingress::Ingress;

// ── json ─────────────────────────────────────────────────────────────────────
pub use json::types::json_codec::JsonCodec;
pub use json::Codec;

// ── metrics ──────────────────────────────────────────────────────────────────
pub use metrics::{MetricsConfig, MetricsExporter, MetricsHandler};

// ── monitor ──────────────────────────────────────────────────────────────────
// MetricsConfig is omitted here — it is already exported via the metrics theme above.
pub use monitor::{
    AutoscalePolicy, GrpcLoadMonitor, HttpLoadMonitor, LifecycleObserver, RingBuffer, Sampler,
    ScalingDecision, ScalingPolicy, SharedCounters, ThresholdPolicy, TrafficCounters,
};

// ── runner ───────────────────────────────────────────────────────────────────
// Runner is also re-exported from runtime theme below; only one declaration needed.

// ── runtime ──────────────────────────────────────────────────────────────────
pub use runtime::{
    ComponentHealth, ConfigValidator, Runner, Runtime, RuntimeBuilder, RuntimeBuilderServe,
    RuntimeError, RuntimeHealth, RuntimeManager, RuntimeResult, RuntimeStatus, ServerConfigLoader,
    ServerMonitor, ServiceRegistry, ServiceRegistryBuilder, TracingInitializer, Validator,
};

// ── validator ────────────────────────────────────────────────────────────────
// (traits already re-exported under runtime theme above)
