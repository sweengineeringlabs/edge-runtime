//! TonicGrpcServer type declaration.

use std::sync::Arc;
use std::time::Duration;

use swe_edge_ingress_grpc::{
    AuditSink, CompressionMode, GrpcIngress, GrpcIngressInterceptorChain, HealthService,
};
use swe_edge_ingress_tls::IngressTlsConfig;

/// Default maximum inbound message size (4 MiB).
pub const MAX_MESSAGE_BYTES: usize = 4 * 1_024 * 1_024;

/// Default HTTP/2 keepalive PING interval (gRPC keepalive spec).
pub const DEFAULT_KEEPALIVE_INTERVAL: Duration = Duration::from_secs(20);

/// Default HTTP/2 keepalive PONG timeout.
pub const DEFAULT_KEEPALIVE_TIMEOUT: Duration = Duration::from_secs(10);

/// Error message when no `AuthorizationInterceptor` is registered and `allow_unauthenticated` is false.
pub const MISSING_AUTHORIZATION_INTERCEPTOR_MSG: &str =
    "gRPC server requires an AuthorizationInterceptor in the chain \
     (e.g. swe-edge-ingress-grpc-authz::AuthzInterceptor). To explicitly run \
     without authz, set `allow_unauthenticated = true` in \
     GrpcServerConfig (logged at startup as a warning).";

/// Sanitized message returned to clients for any `Internal` server error.
pub const SANITIZED_INTERNAL_MSG: &str = "internal server error";

/// Warning logged at startup when gRPC reflection is enabled.
pub const REFLECTION_ENABLED_WARN_MSG: &str =
    "gRPC reflection enabled — exposes service surface to anyone reaching this endpoint. \
     Disable in production deployments.";

/// gRPC server that routes all unary requests through a [`GrpcIngress`] port.
pub struct TonicGrpcServer {
    pub(crate) bind: String,
    pub(crate) handler: Arc<dyn GrpcIngress>,
    pub(crate) max_bytes: usize,
    pub(crate) max_concurrent_streams: u32,
    pub(crate) tls: Option<IngressTlsConfig>,
    pub(crate) interceptors: GrpcIngressInterceptorChain,
    pub(crate) compression: CompressionMode,
    pub(crate) allow_unauthenticated: bool,
    pub(crate) audit_sink: Arc<dyn AuditSink>,
    pub(crate) enable_reflection: bool,
    /// Auto-wired `grpc.health.v1.Health` service. `None` after `.without_health_service()`.
    pub(crate) health_service: Option<Arc<HealthService>>,
    /// When `true`, `TraceContextInterceptor` is prepended to the chain at serve time.
    pub(crate) auto_trace_context: bool,
    /// HTTP/2 keepalive PING interval. `None` disables keepalive.
    pub(crate) keepalive_interval: Option<Duration>,
    /// How long to wait for a PONG before closing the connection.
    pub(crate) keepalive_timeout: Duration,
}
