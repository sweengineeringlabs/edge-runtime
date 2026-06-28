//! Lifecycle management and builder-style configuration for a Tonic gRPC server.

use std::sync::Arc;
use std::time::Duration;

use edge_domain_security::IngressTlsConfig;
use swe_edge_ingress_grpc::{
    AuditSink, CompressionMode, GrpcIngress, GrpcIngressInterceptorChain, HealthService,
};

use crate::api::server::types::GrpcServerConfig;
use crate::api::server::errors::GrpcServerConfigError;

/// Lifecycle management and builder-style configuration for a Tonic gRPC server.
///
/// Consuming methods are gated on `where Self: Sized`; the `&self` accessor methods
/// keep the trait object-safe.
pub trait GrpcServerManage {
    /// Create a server that will bind to `bind` and delegate to `handler`.
    fn new(bind: impl Into<String>, handler: Arc<dyn GrpcIngress>) -> Self
    where
        Self: Sized;
    /// Construct a server from a [`GrpcServerConfig`].
    fn from_config(
        config: &GrpcServerConfig,
        handler: Arc<dyn GrpcIngress>,
    ) -> Result<Self, GrpcServerConfigError>
    where
        Self: Sized;
    /// Enable or disable gRPC server reflection.
    fn enable_reflection(self, enable: bool) -> Self
    where
        Self: Sized;
    /// Returns `true` if gRPC reflection is enabled.
    fn is_reflection_enabled(&self) -> bool;
    /// Replace the default no-op audit sink with a custom implementation.
    fn with_audit_sink(self, sink: Arc<dyn AuditSink>) -> Self
    where
        Self: Sized;
    /// Allow or deny requests without an `AuthorizationInterceptor`.
    fn allow_unauthenticated(self, allow: bool) -> Self
    where
        Self: Sized;
    /// Override the maximum inbound message size in bytes.
    fn with_max_message_size(self, size: usize) -> Self
    where
        Self: Sized;
    /// Override the maximum number of concurrent HTTP/2 streams.
    fn with_max_concurrent_streams(self, streams: u32) -> Self
    where
        Self: Sized;
    /// Attach an interceptor chain that runs before and after each dispatch.
    fn with_interceptors(self, chain: GrpcIngressInterceptorChain) -> Self
    where
        Self: Sized;
    /// Set the response compression mode.
    fn with_compression(self, mode: CompressionMode) -> Self
    where
        Self: Sized;
    /// Attach a TLS configuration (enables mTLS when a CA cert is provided).
    fn with_tls(self, config: IngressTlsConfig) -> Self
    where
        Self: Sized;
    /// Override the HTTP/2 keepalive PING interval and PONG timeout.
    fn with_keepalive(self, interval: Duration, timeout: Duration) -> Self
    where
        Self: Sized;
    /// Disable HTTP/2 keepalive PING frames.
    fn without_keepalive(self) -> Self
    where
        Self: Sized;
    /// Return the configured keepalive interval.
    fn keepalive_interval(&self) -> Option<Duration>;
    /// Return the configured keepalive timeout.
    fn keepalive_timeout(&self) -> Duration;
    /// Disable the auto-wired `TraceContextInterceptor`.
    fn without_trace_context(self) -> Self
    where
        Self: Sized;
    /// Disable the auto-wired `grpc.health.v1.Health` service.
    fn without_health_service(self) -> Self
    where
        Self: Sized;
    /// Access the auto-wired [`HealthService`] to set per-service statuses.
    fn health_service(&self) -> Option<&Arc<HealthService>>;
    /// Replace the auto-wired [`HealthService`] with a caller-provided instance.
    fn with_health_service(self, hs: Arc<HealthService>) -> Self
    where
        Self: Sized;
    /// Return the configured bind address string.
    fn bind_addr(&self) -> &str;
    /// Return the maximum inbound message size in bytes.
    fn max_message_size(&self) -> usize;
    /// Return the maximum number of concurrent HTTP/2 streams.
    fn max_concurrent_streams(&self) -> u32;
    /// Return the TLS configuration, if any.
    fn tls_config(&self) -> Option<&IngressTlsConfig>;
    /// Return the configured compression mode.
    fn compression_mode(&self) -> CompressionMode;
    /// Return `true` if unauthenticated callers are allowed.
    fn is_unauthenticated_allowed(&self) -> bool;
    /// Return `true` if the trace context interceptor is auto-wired.
    fn has_trace_context(&self) -> bool;
    /// Return a reference to the configured audit sink.
    fn audit_sink_ref(&self) -> &Arc<dyn AuditSink>;
    /// Return a reference to the interceptor chain.
    fn interceptor_chain(&self) -> &GrpcIngressInterceptorChain;
}
