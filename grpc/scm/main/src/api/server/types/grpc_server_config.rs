//! Inbound server configuration — TLS-by-default, fail-closed.

use std::net::SocketAddr;

use serde::{Deserialize, Serialize};
use edge_domain_security::IngressTlsConfig;

use swe_edge_ingress_grpc::CompressionMode;

/// Default ceiling for inbound message bytes (4 MiB).
pub const DEFAULT_MAX_MESSAGE_BYTES: usize = 4 * 1024 * 1024;

/// Default cap on concurrent HTTP/2 streams per connection.
pub const DEFAULT_MAX_CONCURRENT_STREAMS: u32 = 100;

/// Default HTTP/2 keepalive PING interval in seconds (gRPC keepalive spec).
pub const DEFAULT_KEEPALIVE_INTERVAL_SECS: u64 = 20;

/// Default HTTP/2 keepalive PONG timeout in seconds.
pub const DEFAULT_KEEPALIVE_TIMEOUT_SECS: u64 = 10;

/// Configuration for an inbound gRPC server.
///
/// **TLS-by-default**.  Plaintext servers must explicitly call
/// [`GrpcServerConfig::allow_plaintext`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrpcServerConfig {
    /// Address to bind.
    pub bind: SocketAddr,
    /// Require TLS on the wire.
    pub tls_required: bool,
    /// TLS configuration.  When set with `client_ca_pem_path`, mTLS.
    pub tls: Option<IngressTlsConfig>,
    /// Hard cap on a single inbound message in bytes.
    pub max_message_bytes: usize,
    /// HTTP/2 SETTINGS_MAX_CONCURRENT_STREAMS advertised to clients.
    pub max_concurrent_streams: u32,
    /// Phase 3 hook — Phase 2 wires the field but does not enforce it.
    pub allow_unauthenticated: bool,
    /// Compression negotiation mode.
    pub compression: CompressionMode,
    /// Phase 5 hook — expose `grpc.reflection.v1alpha.ServerReflection`.
    ///
    /// **Default `false`**.  Reflection lets any caller reaching the
    /// endpoint list every registered method and download
    /// FileDescriptorProto blobs — useful for grpcurl/evans during
    /// development, but a real attack-surface concern in production.
    /// Wiring code that registers a `ReflectionService` MUST gate on
    /// this flag.  The server logs a startup WARN when the flag is
    /// `true` so the decision is observable.
    #[serde(default)]
    pub enable_reflection: bool,
    /// HTTP/2 keepalive PING interval in seconds. `None` disables keepalive.
    #[serde(default = "GrpcServerConfig::default_keepalive_interval_secs")]
    pub keepalive_interval_secs: Option<u64>,
    /// How long to wait for a PONG before considering the connection dead.
    #[serde(default = "GrpcServerConfig::default_keepalive_timeout_secs")]
    pub keepalive_timeout_secs: u64,
}
