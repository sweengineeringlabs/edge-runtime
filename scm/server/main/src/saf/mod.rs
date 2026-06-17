//! SAF layer — daemon public facade.

mod config;
pub mod daemon;
mod lifecycle;
mod metrics;
mod runtime;
mod server_svc;

// ── _svc modules (rule 218) ───────────────────────────────────────────────────
mod application_config_loader_svc;
mod codec_svc;
mod composite_ingress_svc;
mod egress_svc;
mod grpc_load_monitor_svc;
mod http_load_monitor_svc;
mod ingress_svc;
mod json_codec_svc;
mod runner_svc;
mod sampler_svc;
mod scaling_policy_svc;
mod validator_svc;

pub use runtime::*;
pub use server_svc::*;

// Traits + SVC consts from grouped subdirectory modules
pub use application_config_loader_svc::{ApplicationConfigLoader, APPLICATION_CONFIG_LOADER_SVC};
pub use codec_svc::{Codec, CODEC_SVC};
pub use composite_ingress_svc::{CompositeGrpcIngress, CompositeIngress, COMPOSITE_INGRESS_SVC};
pub use config::{ConfigValidator, CONFIG_LOADER_SVC, CONFIG_VALIDATOR_SVC};
pub use grpc_load_monitor_svc::{GrpcLoadMonitor, GRPC_LOAD_MONITOR_SVC};
pub use http_load_monitor_svc::{HttpLoadMonitor, HTTP_LOAD_MONITOR_SVC};
pub use json_codec_svc::{JsonCodec, JSON_CODEC_SVC};
pub use lifecycle::LIFECYCLE_OBSERVER_SVC;
pub use metrics::{
    MetricsExporter, MetricsHandler as MetricsHandlerTrait, METRICS_EXPORTER_SVC,
    METRICS_HANDLER_SVC,
};
pub use runner_svc::{Runner, RuntimeBuilderServe, RUNNER_SVC};
pub use runtime::RUNTIME_MANAGER_SVC;
pub use sampler_svc::{Sampler, SAMPLER_SVC};
pub use validator_svc::{Validator, VALIDATOR_SVC};

// SVC consts for traits already exported via runtime::*
pub use egress_svc::EGRESS_SVC;
pub use ingress_svc::INGRESS_SVC;
pub use scaling_policy_svc::SCALING_POLICY_SVC;

// ── Auth / TLS ────────────────────────────────────────────────────────────────
pub use swe_edge_ingress_grpc::{
    AuthorizationInterceptor, GrpcIngressInterceptor, GrpcIngressInterceptorChain,
};
pub use swe_edge_ingress_verifier::{
    Claims, JwtConfig, JwtKey, JwtVerifier, TokenVerifier, VerifierError,
};

// ── Ingress surface (handlers + request/response types) ───────────────────────
pub use edge_domain::{Handler, HandlerContext, HandlerError, SecurityContext};
pub use swe_edge_ingress_grpc::{
    GrpcDecodeFn, GrpcEncodeFn, GrpcHealthCheck, GrpcIngress, GrpcIngressError, GrpcIngressResult,
    GrpcMessageStream, GrpcMetadata, GrpcRequest, GrpcResponse, GrpcStatusCode,
};
pub use swe_edge_ingress_http::{
    HttpAuth, HttpBody, HttpConfig, HttpDecodeFn, HttpEncodeFn, HttpHealthCheck, HttpIngress,
    HttpIngressError, HttpIngressResult, HttpMethod, HttpRequest, HttpResponse, HttpStream,
    IngressTlsConfig,
};

// ── Egress surface (outbound clients) ─────────────────────────────────────────
pub use swe_edge_egress_grpc::{GrpcEgress, GrpcEgressError, GrpcEgressResult, TonicGrpcClient};
pub use swe_edge_egress_http::{HttpEgress, HttpEgressError, HttpEgressResult, HttpStreamResponse};

// ── Lifecycle / health ────────────────────────────────────────────────────────
pub use edge_proxy::{HealthReport, LifecycleMonitor, ProxySvc};

pub use swe_observ_metrics::{MetricSnapshot, MetricType, MetricsProvider};

// ── Observability ─────────────────────────────────────────────────────────────
pub use swe_edge_observ_config::{ObservabilityConfig, TracingConfig, TracingFormat, TracingLevel};

// ── Message broker ────────────────────────────────────────────────────────────
#[cfg(feature = "message-broker")]
pub use swe_edge_runtime_message_broker::MessageBrokerFactory;
#[cfg(feature = "message-broker")]
pub use swe_edge_runtime_message_broker::{Message, MessageBroker, MessageStream};

// ── Subprocess ────────────────────────────────────────────────────────────────
#[cfg(feature = "subprocess")]
pub use swe_edge_egress_subprocess::{
    SubprocessArgs, SubprocessArgsBuilder, SubprocessConfig, SubprocessConfigBuilder,
    SubprocessResult, SubprocessRunner, SubprocessRunnerExtension, SubprocessSvc,
};
