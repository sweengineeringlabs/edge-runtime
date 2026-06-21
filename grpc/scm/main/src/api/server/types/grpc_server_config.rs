//! Inbound server configuration — TLS-by-default, fail-closed.

use std::net::SocketAddr;

use serde::{Deserialize, Serialize};
use swe_edge_ingress_tls::IngressTlsConfig;

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

impl GrpcServerConfig {
    /// Default keepalive interval used by serde when the field is absent.
    pub fn default_keepalive_interval_secs() -> Option<u64> {
        Some(DEFAULT_KEEPALIVE_INTERVAL_SECS)
    }

    /// Default keepalive timeout used by serde when the field is absent.
    pub fn default_keepalive_timeout_secs() -> u64 {
        DEFAULT_KEEPALIVE_TIMEOUT_SECS
    }

    /// Construct a server config bound to `bind` with TLS required and
    /// all other knobs at fail-closed defaults.
    pub fn new(bind: SocketAddr) -> Self {
        Self {
            bind,
            tls_required: true,
            tls: None,
            max_message_bytes: DEFAULT_MAX_MESSAGE_BYTES,
            max_concurrent_streams: DEFAULT_MAX_CONCURRENT_STREAMS,
            allow_unauthenticated: false,
            compression: CompressionMode::None,
            enable_reflection: false,
            keepalive_interval_secs: Some(DEFAULT_KEEPALIVE_INTERVAL_SECS),
            keepalive_timeout_secs: DEFAULT_KEEPALIVE_TIMEOUT_SECS,
        }
    }

    /// Explicitly relax the TLS requirement.
    pub fn allow_plaintext(mut self) -> Self {
        self.tls_required = false;
        self
    }

    /// Attach a TLS / mTLS configuration.
    pub fn with_tls(mut self, tls: IngressTlsConfig) -> Self {
        self.tls = Some(tls);
        self
    }

    /// Override the max-message-bytes cap.
    pub fn with_max_message_bytes(mut self, bytes: usize) -> Self {
        self.max_message_bytes = bytes;
        self
    }

    /// Override the max-concurrent-streams cap.
    pub fn with_max_concurrent_streams(mut self, streams: u32) -> Self {
        self.max_concurrent_streams = streams;
        self
    }

    /// Set the compression mode.
    pub fn with_compression(mut self, mode: CompressionMode) -> Self {
        self.compression = mode;
        self
    }

    /// Override the HTTP/2 keepalive interval and PONG timeout.
    ///
    /// Set `interval_secs = 0` to disable keepalive entirely.
    pub fn with_keepalive(mut self, interval_secs: u64, timeout_secs: u64) -> Self {
        self.keepalive_interval_secs = if interval_secs == 0 {
            None
        } else {
            Some(interval_secs)
        };
        self.keepalive_timeout_secs = timeout_secs;
        self
    }

    /// Disable HTTP/2 keepalive PING frames.
    pub fn without_keepalive(mut self) -> Self {
        self.keepalive_interval_secs = None;
        self
    }

    /// Phase 3 hook: opt out of authentication.
    pub fn allow_unauthenticated(mut self) -> Self {
        self.allow_unauthenticated = true;
        self
    }

    /// Phase 5 hook: opt in to gRPC reflection.
    pub fn enable_reflection(mut self) -> Self {
        self.enable_reflection = true;
        self
    }
}

impl swe_edge_configbuilder::ConfigSection for GrpcServerConfig {
    fn section_name() -> &'static str {
        const NAME: &str = "grpc";
        NAME
    }
}

impl Default for GrpcServerConfig {
    fn default() -> Self {
        Self {
            bind: SocketAddr::from(([0, 0, 0, 0], 0)),
            tls_required: true,
            tls: None,
            max_message_bytes: DEFAULT_MAX_MESSAGE_BYTES,
            max_concurrent_streams: DEFAULT_MAX_CONCURRENT_STREAMS,
            allow_unauthenticated: false,
            compression: CompressionMode::None,
            enable_reflection: false,
            keepalive_interval_secs: Some(DEFAULT_KEEPALIVE_INTERVAL_SECS),
            keepalive_timeout_secs: DEFAULT_KEEPALIVE_TIMEOUT_SECS,
        }
    }
}
