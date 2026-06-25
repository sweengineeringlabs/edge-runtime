//! GrpcServerConfig trait implementations (Default, ConfigSection).

use std::net::SocketAddr;

use swe_edge_ingress_grpc::CompressionMode;

use crate::api::{
    GrpcServerConfig, DEFAULT_KEEPALIVE_INTERVAL_SECS, DEFAULT_KEEPALIVE_TIMEOUT_SECS,
    DEFAULT_MAX_CONCURRENT_STREAMS, DEFAULT_MAX_MESSAGE_BYTES,
};

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
