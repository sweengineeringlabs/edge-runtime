//! GrpcServerConfigOps trait impl for GrpcServerConfig.
use std::net::SocketAddr;

use edge_domain_security::PemTlsConfig;
use swe_edge_ingress_grpc::CompressionMode;

use crate::api::{
    GrpcServerConfig, GrpcServerConfigOps, DEFAULT_KEEPALIVE_INTERVAL_SECS,
    DEFAULT_KEEPALIVE_TIMEOUT_SECS, DEFAULT_MAX_CONCURRENT_STREAMS, DEFAULT_MAX_MESSAGE_BYTES,
};

impl GrpcServerConfigOps for GrpcServerConfig {
    fn default_keepalive_interval_secs() -> Option<u64> {
        Some(DEFAULT_KEEPALIVE_INTERVAL_SECS)
    }

    fn default_keepalive_timeout_secs() -> u64 {
        DEFAULT_KEEPALIVE_TIMEOUT_SECS
    }

    fn new(bind: SocketAddr) -> Self {
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

    fn allow_plaintext(mut self) -> Self {
        self.tls_required = false;
        self
    }

    fn with_tls(mut self, tls: PemTlsConfig) -> Self {
        self.tls = Some(tls);
        self
    }

    fn with_max_message_bytes(mut self, bytes: usize) -> Self {
        self.max_message_bytes = bytes;
        self
    }

    fn with_max_concurrent_streams(mut self, streams: u32) -> Self {
        self.max_concurrent_streams = streams;
        self
    }

    fn with_compression(mut self, mode: CompressionMode) -> Self {
        self.compression = mode;
        self
    }

    fn with_keepalive(mut self, interval_secs: u64, timeout_secs: u64) -> Self {
        self.keepalive_interval_secs = if interval_secs == 0 {
            None
        } else {
            Some(interval_secs)
        };
        self.keepalive_timeout_secs = timeout_secs;
        self
    }

    fn without_keepalive(mut self) -> Self {
        self.keepalive_interval_secs = None;
        self
    }

    fn allow_unauthenticated(mut self) -> Self {
        self.allow_unauthenticated = true;
        self
    }

    fn enable_reflection(mut self) -> Self {
        self.enable_reflection = true;
        self
    }

    fn bind_addr(&self) -> &SocketAddr {
        &self.bind
    }
}
