//! Fluent builder interface for a Tonic gRPC server.

use std::sync::Arc;

use edge_domain_security::PemTlsConfig;
use swe_edge_ingress_grpc::{AuditSink, CompressionMode, GrpcIngress, GrpcIngressInterceptorChain};

use crate::api::TonicGrpcServer;

/// Fluent builder interface for constructing a [`TonicGrpcServer`].
///
/// Consuming methods are gated on `where Self: Sized`; [`GrpcServerBuild::builder_bind`]
/// keeps the trait object-safe.
pub trait GrpcServerBuild {
    /// Start building a server bound to `bind` that delegates to `handler`.
    fn new(bind: impl Into<String>, handler: Arc<dyn GrpcIngress>) -> Self
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
    /// Attach a TLS configuration.
    fn with_tls(self, cfg: PemTlsConfig) -> Self
    where
        Self: Sized;
    /// Attach an interceptor chain.
    fn with_interceptors(self, chain: GrpcIngressInterceptorChain) -> Self
    where
        Self: Sized;
    /// Set the compression mode.
    fn with_compression(self, mode: CompressionMode) -> Self
    where
        Self: Sized;
    /// Allow unauthenticated callers.
    fn allow_unauthenticated(self) -> Self
    where
        Self: Sized;
    /// Replace the default no-op audit sink.
    fn with_audit_sink(self, sink: Arc<dyn AuditSink>) -> Self
    where
        Self: Sized;
    /// Enable gRPC reflection.
    fn enable_reflection(self) -> Self
    where
        Self: Sized;
    /// Consume the builder and produce a [`TonicGrpcServer`].
    fn build(self) -> TonicGrpcServer
    where
        Self: Sized;
    /// Return the configured bind string (object-safe accessor).
    fn builder_bind(&self) -> &str;
}
