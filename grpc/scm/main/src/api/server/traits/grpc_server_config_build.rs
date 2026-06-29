//! Fluent builder interface for gRPC server configuration.

use std::net::SocketAddr;

use edge_domain_security::PemTlsConfig;
use swe_edge_ingress_grpc::CompressionMode;

use crate::api::GrpcServerConfig;

/// Fluent builder interface for constructing a [`GrpcServerConfig`].
///
/// Implement this trait to provide builder-pattern construction.  All consuming
/// methods are gated on `where Self: Sized` to preserve object-safety via
/// [`bind_addr`].
pub trait GrpcServerConfigBuild {
    /// Start a new builder bound to `bind`.
    fn new(bind: SocketAddr) -> Self
    where
        Self: Sized;
    /// Allow plaintext (clears the TLS-required flag).
    fn allow_plaintext(self) -> Self
    where
        Self: Sized;
    /// Attach TLS configuration.
    fn with_tls(self, tls: PemTlsConfig) -> Self
    where
        Self: Sized;
    /// Override the max-message-bytes cap.
    fn with_max_message_bytes(self, bytes: usize) -> Self
    where
        Self: Sized;
    /// Override the max-concurrent-streams cap.
    fn with_max_concurrent_streams(self, streams: u32) -> Self
    where
        Self: Sized;
    /// Allow unauthenticated callers.
    fn allow_unauthenticated(self) -> Self
    where
        Self: Sized;
    /// Set the compression mode.
    fn with_compression(self, mode: CompressionMode) -> Self
    where
        Self: Sized;
    /// Enable gRPC reflection.
    fn enable_reflection(self) -> Self
    where
        Self: Sized;
    /// Override the HTTP/2 keepalive interval and PONG timeout (in seconds).
    fn with_keepalive(self, interval_secs: u64, timeout_secs: u64) -> Self
    where
        Self: Sized;
    /// Disable HTTP/2 keepalive PING frames.
    fn without_keepalive(self) -> Self
    where
        Self: Sized;
    /// Consume the builder and produce a [`GrpcServerConfig`].
    fn build(self) -> GrpcServerConfig
    where
        Self: Sized;
    /// Return the configured bind address (object-safe accessor).
    fn bind_addr(&self) -> &SocketAddr;
}
