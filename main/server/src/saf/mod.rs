//! SAF layer — daemon public facade.

mod server_svc;

pub use crate::api::config::loader::ConfigLoader;
pub use crate::api::config::ConfigError;
pub use crate::api::egress::Egress;
pub use crate::api::error::{RuntimeError, RuntimeResult};
pub use crate::api::ingress::Ingress;
pub use crate::api::runtime::manager::RuntimeManager;
pub use crate::api::runtime::{Runtime, RuntimeBuilder};
pub use crate::api::types::runtime::health::ComponentHealth;
pub use crate::api::types::ServiceRegistry;
pub use crate::api::types::{RuntimeConfig, RuntimeHealth, RuntimeStatus};
pub use crate::api::types::{ServerConfigLoader, ServerMonitor};

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
    GrpcDecodeFn, GrpcEncodeFn, GrpcHealthCheck, GrpcIngress, GrpcIngressError, GrpcIngressResult,
    GrpcMessageStream, GrpcMetadata, GrpcRequest, GrpcResponse, GrpcStatusCode,
};
pub use swe_edge_ingress_http::{
    HttpAuth, HttpBody, HttpConfig, HttpDecodeFn, HttpEncodeFn, HttpHealthCheck, HttpIngress,
    HttpIngressError, HttpIngressResult, HttpMethod, HttpRequest, HttpResponse, HttpStream,
    IngressTlsConfig, RequestContext,
};

// ── Egress surface (outbound clients) ─────────────────────────────────────────
pub use swe_edge_egress_grpc::{GrpcEgress, GrpcEgressError, GrpcEgressResult, TonicGrpcClient};
pub use swe_edge_egress_http::{HttpEgress, HttpEgressError, HttpEgressResult, HttpStreamResponse};

// ── Lifecycle / health ────────────────────────────────────────────────────────
pub use edge_proxy::{new_null_lifecycle_monitor, HealthReport, LifecycleMonitor};

// ── Load monitoring / auto-scaling ────────────────────────────────────────────
pub use crate::api::monitor::ring_buffer::RingBuffer;
pub use crate::api::monitor::{AutoscalePolicy, MetricsConfig, SharedCounters, TrafficCounters};
pub use swe_observ_metrics::{MetricSnapshot, MetricType, MetricsProvider};

// ── Observability ─────────────────────────────────────────────────────────────
#[cfg(feature = "observability")]
pub use crate::api::types::TracingInitializer;
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
