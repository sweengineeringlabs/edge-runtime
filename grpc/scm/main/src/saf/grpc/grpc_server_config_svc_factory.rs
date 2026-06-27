//! SAF factory surface — GrpcServerConfig inherent methods.

use std::net::SocketAddr;

use swe_edge_ingress_grpc::CompressionMode;
use edge_domain_security::IngressTlsConfig;

use crate::api::{
    GrpcServerConfig, DEFAULT_KEEPALIVE_INTERVAL_SECS, DEFAULT_KEEPALIVE_TIMEOUT_SECS,
    DEFAULT_MAX_CONCURRENT_STREAMS, DEFAULT_MAX_MESSAGE_BYTES,
};

impl GrpcServerConfig {
    /// Default keepalive interval used by serde when the field is absent.
    pub fn default_keepalive_interval_secs() -> Option<u64> {
        Some(DEFAULT_KEEPALIVE_INTERVAL_SECS)
    }

    /// Default keepalive timeout used by serde when the field is absent.
    pub fn default_keepalive_timeout_secs() -> u64 {
        DEFAULT_KEEPALIVE_TIMEOUT_SECS
    }

    /// Construct a server config bound to `bind` with TLS required and fail-closed defaults.
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
