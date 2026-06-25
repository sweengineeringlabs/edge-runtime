//! SAF factory surface — GrpcServerConfigBuilder inherent methods.

use std::net::SocketAddr;

use swe_edge_ingress_grpc::CompressionMode;
use swe_edge_ingress_tls::IngressTlsConfig;

use crate::api::{
    GrpcServerConfig, GrpcServerConfigBuilder, DEFAULT_KEEPALIVE_INTERVAL_SECS,
    DEFAULT_KEEPALIVE_TIMEOUT_SECS, DEFAULT_MAX_CONCURRENT_STREAMS, DEFAULT_MAX_MESSAGE_BYTES,
};

impl GrpcServerConfigBuilder {
    /// Start building a configuration bound to `bind` with TLS required by default.
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

    /// Relax the TLS requirement to allow plaintext connections.
    pub fn allow_plaintext(mut self) -> Self {
        self.tls_required = false;
        self
    }

    /// Attach TLS configuration.
    pub fn with_tls(mut self, tls: IngressTlsConfig) -> Self {
        self.tls = Some(tls);
        self
    }

    /// Override the max message bytes cap.
    pub fn with_max_message_bytes(mut self, bytes: usize) -> Self {
        self.max_message_bytes = bytes;
        self
    }

    /// Override the max concurrent streams cap.
    pub fn with_max_concurrent_streams(mut self, streams: u32) -> Self {
        self.max_concurrent_streams = streams;
        self
    }

    /// Allow unauthenticated callers.
    pub fn allow_unauthenticated(mut self) -> Self {
        self.allow_unauthenticated = true;
        self
    }

    /// Set the compression mode.
    pub fn with_compression(mut self, mode: CompressionMode) -> Self {
        self.compression = mode;
        self
    }

    /// Enable gRPC reflection.
    pub fn enable_reflection(mut self) -> Self {
        self.enable_reflection = true;
        self
    }

    /// Override the HTTP/2 keepalive interval and PONG timeout (seconds).
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

    /// Build the [`GrpcServerConfig`].
    pub fn build(self) -> GrpcServerConfig {
        GrpcServerConfig {
            bind: self.bind,
            tls_required: self.tls_required,
            tls: self.tls,
            max_message_bytes: self.max_message_bytes,
            max_concurrent_streams: self.max_concurrent_streams,
            allow_unauthenticated: self.allow_unauthenticated,
            compression: self.compression,
            enable_reflection: self.enable_reflection,
            keepalive_interval_secs: self.keepalive_interval_secs,
            keepalive_timeout_secs: self.keepalive_timeout_secs,
        }
    }
}
