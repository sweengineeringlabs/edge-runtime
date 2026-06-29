//! Builder-style operations on a gRPC server configuration.

use std::net::SocketAddr;

use edge_domain_security::PemTlsConfig;
use swe_edge_ingress_grpc::CompressionMode;

use crate::api::GrpcServerConfig;

/// Builder-style operations on a [`GrpcServerConfig`] value.
///
/// All consuming mutator methods are gated on `where Self: Sized` to keep the
/// trait object-safe (via [`bind_addr`]).  Static `fn new / default_*` helpers
/// are also `where Self: Sized`.
pub trait GrpcServerConfigOps {
    /// Default keepalive interval returned by serde when the field is absent.
    fn default_keepalive_interval_secs() -> Option<u64>
    where
        Self: Sized;
    /// Default keepalive timeout returned by serde when the field is absent.
    fn default_keepalive_timeout_secs() -> u64
    where
        Self: Sized;
    /// Construct a TLS-required, fail-closed config bound to `bind`.
    fn new(bind: SocketAddr) -> Self
    where
        Self: Sized;
    /// Relax the TLS requirement to allow plaintext.
    fn allow_plaintext(self) -> Self
    where
        Self: Sized;
    /// Attach a TLS / mTLS configuration.
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
    /// Set the compression mode.
    fn with_compression(self, mode: CompressionMode) -> Self
    where
        Self: Sized;
    /// Override the HTTP/2 keepalive interval and PONG timeout (seconds).
    fn with_keepalive(self, interval_secs: u64, timeout_secs: u64) -> Self
    where
        Self: Sized;
    /// Disable HTTP/2 keepalive PING frames.
    fn without_keepalive(self) -> Self
    where
        Self: Sized;
    /// Allow unauthenticated callers.
    fn allow_unauthenticated(self) -> Self
    where
        Self: Sized;
    /// Opt in to gRPC reflection (off by default).
    fn enable_reflection(self) -> Self
    where
        Self: Sized;
    /// Return the configured bind address (object-safe accessor).
    fn bind_addr(&self) -> &SocketAddr;
}
