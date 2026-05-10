//! SAF layer — daemon public facade.

pub mod config_loader;
pub mod daemon;
/// SAF factory for wrapping a [`LifecycleMonitor`] with metrics observation.
pub mod lifecycle_monitor;

pub use crate::api::config::ConfigError;
pub use crate::api::config_loader::ConfigLoader;
pub use crate::api::runtime::{Runtime, RuntimeBuilder};
pub use crate::api::error::{RuntimeError, RuntimeResult};
pub use crate::api::runtime_manager::RuntimeManager;
pub use crate::api::service_registry::ServiceRegistry;
pub use crate::api::types::{RuntimeConfig, RuntimeHealth, RuntimeStatus};
pub use crate::api::types::runtime_health::ComponentHealth;
pub use crate::api::input::{DefaultInput, Input};
pub use crate::api::output::{DefaultOutput, Output};

// ── Auth / TLS ────────────────────────────────────────────────────────────────
pub use swe_edge_ingress_verifier::{TokenVerifier, JwtVerifier, JwtConfig, JwtKey, Claims, VerifierError};
pub use swe_edge_ingress::{
    GrpcInboundInterceptor, AuthorizationInterceptor, GrpcInboundInterceptorChain,
};

// ── Ingress surface (handlers + request/response types) ───────────────────────
pub use swe_edge_ingress::{
    Handler, HandlerError,
    HttpDecodeFn, HttpEncodeFn,
    GrpcDecodeFn, GrpcEncodeFn,
    HttpRequest, HttpResponse, HttpInboundError, HttpInboundResult, HttpHealthCheck,
    HttpMethod, HttpAuth, HttpBody, HttpConfig,
    GrpcRequest, GrpcResponse, GrpcInboundError, GrpcInboundResult, GrpcHealthCheck,
    GrpcMetadata, GrpcStatusCode, GrpcMessageStream,
    RequestContext,
    IngressTlsConfig,
};

// ── Egress surface (outbound clients) ─────────────────────────────────────────
pub use swe_edge_egress_http::{
    HttpOutbound, HttpOutboundError, HttpOutboundResult, HttpStreamResponse,
};
pub use swe_edge_egress_grpc::{GrpcOutbound, GrpcOutboundError, GrpcOutboundResult, TonicGrpcClient};

// ── Lifecycle / health ────────────────────────────────────────────────────────
pub use edge_proxy::{LifecycleMonitor, HealthReport, new_null_lifecycle_monitor};
pub use lifecycle_monitor::observe_lifecycle_monitor;

// ── Load monitoring / auto-scaling ────────────────────────────────────────────
pub use crate::api::monitor::{
    AutoscalePolicy, TrafficCounters, MetricsConfig, SharedCounters,
};
pub use swe_observ_metrics::{MetricsProvider, MetricSnapshot, MetricType};

// ── Config loaders ────────────────────────────────────────────────────────────
pub use config_loader::{
    load_config, load_config_from,
    load_tenant_config, load_tenant_config_from,
    load_config_xdg, load_tenant_config_xdg,
    validate_config,
};

// ── Observability ─────────────────────────────────────────────────────────────
#[cfg(feature = "observability")]
pub use crate::api::tracing_format::TracingFormat;
#[cfg(feature = "observability")]
pub use crate::api::observability::init_tracing;

// ── Daemon runner ─────────────────────────────────────────────────────────────
pub use daemon::{run, runtime_manager};
