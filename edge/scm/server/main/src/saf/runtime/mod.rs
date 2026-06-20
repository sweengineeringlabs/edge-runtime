//! SAF — runtime service surface.
mod runtime_manager_svc;
pub mod runtime_svc;
pub use runtime_manager_svc::RUNTIME_MANAGER_SVC;
#[cfg(feature = "observability")]
pub use runtime_svc::TracingInitializer;
pub use runtime_svc::{
    AutoscalePolicy, ComponentHealth, ConfigError, ConfigLoader, Egress, Ingress, MetricsConfig,
    RingBuffer, Runtime, RuntimeBuilder, RuntimeConfig, RuntimeError, RuntimeHealth,
    RuntimeManager, RuntimeResult, RuntimeStatus, ScalingDecision, ScalingPolicy,
    ServerConfigLoader, ServerMonitor, ServiceRegistry, ServiceRegistryBuilder, SharedCounters,
    ThresholdPolicy, TrafficCounters, RUNTIME_API_VERSION,
};
