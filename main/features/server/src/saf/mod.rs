//! SAF layer — daemon public facade.

pub mod config_loader;
pub mod daemon;
/// SAF factory for wrapping a [`LifecycleMonitor`] with metrics observation.
pub mod lifecycle_monitor;

pub use crate::api::config::ConfigError;
pub use crate::api::config_loader::ConfigLoader;
pub use crate::api::egress::{DefaultEgress, Egress};
pub use crate::api::error::{RuntimeError, RuntimeResult};
pub use crate::api::ingress::{DefaultIngress, Ingress};
pub use crate::api::runtime::{Runtime, RuntimeBuilder};
pub use crate::api::runtime_manager::RuntimeManager;
pub use crate::api::service_registry::ServiceRegistry;
pub use crate::api::types::runtime_health::ComponentHealth;
pub use crate::api::types::{RuntimeConfig, RuntimeHealth, RuntimeStatus};
// ── Auth / TLS ────────────────────────────────────────────────────────────────
pub use swe_edge_ingress_grpc::{
    AuthorizationInterceptor, GrpcIngressInterceptor, GrpcIngressInterceptorChain,
};
pub use swe_edge_ingress_verifier::{
    Claims, JwtConfig, JwtKey, JwtVerifier, TokenVerifier, VerifierError,
};

// ── Ingress surface (handlers + request/response types) ───────────────────────
pub use edge_domain::{Handler, HandlerError};
pub use swe_edge_ingress_grpc::{
    GrpcDecodeFn, GrpcEncodeFn, GrpcHealthCheck, GrpcIngressError, GrpcIngressResult,
    GrpcMessageStream, GrpcMetadata, GrpcRequest, GrpcResponse, GrpcStatusCode,
};
pub use swe_edge_ingress_http::{
    HttpAuth, HttpBody, HttpConfig, HttpDecodeFn, HttpEncodeFn, HttpHealthCheck, HttpIngressError,
    HttpIngressResult, HttpMethod, HttpRequest, HttpResponse, IngressTlsConfig, RequestContext,
};

// ── Egress surface (outbound clients) ─────────────────────────────────────────
pub use swe_edge_egress_grpc::{GrpcEgress, GrpcEgressError, GrpcEgressResult, TonicGrpcClient};
pub use swe_edge_egress_http::{HttpEgress, HttpEgressError, HttpEgressResult, HttpStreamResponse};

// ── Lifecycle / health ────────────────────────────────────────────────────────
pub use edge_proxy::{new_null_lifecycle_monitor, HealthReport, LifecycleMonitor};
pub use lifecycle_monitor::observe_lifecycle_monitor;

// ── Load monitoring / auto-scaling ────────────────────────────────────────────
pub use crate::api::monitor::{AutoscalePolicy, MetricsConfig, SharedCounters, TrafficCounters};
pub use swe_observ_metrics::{MetricSnapshot, MetricType, MetricsProvider};

// ── Config loaders ────────────────────────────────────────────────────────────
pub use config_loader::{
    create_config_builder, load_config, load_config_from, load_config_xdg, load_section,
    load_section_from, load_section_xdg, load_tenant_config, load_tenant_config_from,
    load_tenant_config_xdg, validate_config,
};

// ── Observability ─────────────────────────────────────────────────────────────
#[cfg(feature = "observability")]
pub use crate::api::observability::init_tracing;
pub use swe_edge_observ_config::{ObservabilityConfig, TracingConfig, TracingFormat, TracingLevel};

// ── Scheduler ─────────────────────────────────────────────────────────────────
#[cfg(feature = "scheduler")]
pub use swe_edge_runtime_scheduler::{
    tokio_scheduler, Scheduler, SchedulerResult, TokioSchedulerConfig,
};

// ── Message broker ────────────────────────────────────────────────────────────
#[cfg(feature = "message-broker")]
pub use swe_edge_runtime_message_broker::in_memory_broker;
#[cfg(feature = "message-broker")]
pub use swe_edge_runtime_message_broker::{Message, MessageBroker, MessageStream};

// ── Daemon runner ─────────────────────────────────────────────────────────────
pub use daemon::{run, runtime_manager};
