//! TonicGrpcServer type declaration — SEA api/ is the legal declaration site.

use std::sync::Arc;
use std::time::Duration;

use swe_edge_ingress_tls::IngressTlsConfig;
use swe_edge_ingress_grpc::{
    AuditSink, CompressionMode, GrpcIngress, GrpcIngressInterceptorChain, HealthService,
    NoopAuditSink,
};

use crate::api::server::error::GrpcServerConfigError;
use crate::api::server::types::GrpcServerConfig;

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

impl TonicGrpcServer {
    /// Create a server that will bind to `bind` and delegate to `handler`.
    pub fn new(bind: impl Into<String>, handler: Arc<dyn GrpcIngress>) -> Self {
        Self {
            bind: bind.into(),
            handler,
            max_bytes: MAX_MESSAGE_BYTES,
            max_concurrent_streams: 100,
            tls: None,
            interceptors: GrpcIngressInterceptorChain::new(),
            compression: CompressionMode::None,
            allow_unauthenticated: false,
            audit_sink: Arc::new(NoopAuditSink),
            enable_reflection: false,
            health_service: Some(Arc::new(HealthService::new())),
            auto_trace_context: true,
            keepalive_interval: Some(DEFAULT_KEEPALIVE_INTERVAL),
            keepalive_timeout: DEFAULT_KEEPALIVE_TIMEOUT,
        }
    }

    /// Construct a server from a [`GrpcServerConfig`].
    pub fn from_config(
        config: &GrpcServerConfig,
        handler: Arc<dyn GrpcIngress>,
    ) -> Result<Self, GrpcServerConfigError> {
        if config.tls_required && config.tls.is_none() {
            return Err(GrpcServerConfigError::TlsRequiredButMissing);
        }
        Ok(Self {
            bind: config.bind.to_string(),
            handler,
            max_bytes: config.max_message_bytes,
            max_concurrent_streams: config.max_concurrent_streams,
            tls: config.tls.clone(),
            interceptors: GrpcIngressInterceptorChain::new(),
            compression: config.compression,
            allow_unauthenticated: config.allow_unauthenticated,
            audit_sink: Arc::new(NoopAuditSink),
            enable_reflection: config.enable_reflection,
            health_service: Some(Arc::new(HealthService::new())),
            auto_trace_context: true,
            keepalive_interval: config.keepalive_interval_secs.map(Duration::from_secs),
            keepalive_timeout: Duration::from_secs(config.keepalive_timeout_secs),
        })
    }

    /// Enable or disable gRPC server reflection.
    pub fn enable_reflection(mut self, enable: bool) -> Self {
        self.enable_reflection = enable;
        self
    }

    /// Returns `true` if gRPC reflection is enabled.
    pub fn is_reflection_enabled(&self) -> bool {
        self.enable_reflection
    }

    /// Replace the default no-op audit sink with a custom implementation.
    pub fn with_audit_sink(mut self, sink: Arc<dyn AuditSink>) -> Self {
        self.audit_sink = sink;
        self
    }

    /// Allow requests without an `AuthorizationInterceptor` in the chain.
    pub fn allow_unauthenticated(mut self, allow: bool) -> Self {
        self.allow_unauthenticated = allow;
        self
    }

    /// Override the maximum inbound message size in bytes.
    pub fn with_max_message_size(mut self, size: usize) -> Self {
        self.max_bytes = size;
        self
    }

    /// Override the maximum number of concurrent HTTP/2 streams.
    pub fn with_max_concurrent_streams(mut self, streams: u32) -> Self {
        self.max_concurrent_streams = streams;
        self
    }

    /// Attach an interceptor chain that runs before and after each dispatch.
    pub fn with_interceptors(mut self, chain: GrpcIngressInterceptorChain) -> Self {
        self.interceptors = chain;
        self
    }

    /// Set the response compression mode.
    pub fn with_compression(mut self, mode: CompressionMode) -> Self {
        self.compression = mode;
        self
    }

    /// Attach a TLS configuration (enables mTLS when a CA cert is provided).
    pub fn with_tls(mut self, config: IngressTlsConfig) -> Self {
        self.tls = Some(config);
        self
    }

    /// Override the HTTP/2 keepalive PING interval and PONG timeout.
    pub fn with_keepalive(mut self, interval: Duration, timeout: Duration) -> Self {
        self.keepalive_interval = Some(interval);
        self.keepalive_timeout = timeout;
        self
    }

    /// Disable HTTP/2 keepalive PING frames.
    pub fn without_keepalive(mut self) -> Self {
        self.keepalive_interval = None;
        self
    }

    /// Return the configured keepalive interval.
    pub fn keepalive_interval(&self) -> Option<Duration> {
        self.keepalive_interval
    }

    /// Return the configured keepalive timeout.
    pub fn keepalive_timeout(&self) -> Duration {
        self.keepalive_timeout
    }

    /// Disable the auto-wired `TraceContextInterceptor`.
    pub fn without_trace_context(mut self) -> Self {
        self.auto_trace_context = false;
        self
    }

    /// Disable the auto-wired `grpc.health.v1.Health` service.
    pub fn without_health_service(mut self) -> Self {
        self.health_service = None;
        self
    }

    /// Access the auto-wired [`HealthService`] to set per-service statuses.
    pub fn health_service(&self) -> Option<&Arc<HealthService>> {
        self.health_service.as_ref()
    }

    /// Replace the auto-wired [`HealthService`] with a caller-provided instance.
    pub fn with_health_service(mut self, hs: Arc<HealthService>) -> Self {
        self.health_service = Some(hs);
        self
    }
}
