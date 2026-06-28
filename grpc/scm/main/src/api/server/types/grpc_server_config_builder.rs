//! Builder for GrpcServerConfig.

use std::net::SocketAddr;

use edge_domain_security::IngressTlsConfig;
use swe_edge_ingress_grpc::CompressionMode;

/// Fluent builder for [`GrpcServerConfig`].
pub struct GrpcServerConfigBuilder {
    pub(crate) bind: SocketAddr,
    pub(crate) tls_required: bool,
    pub(crate) tls: Option<IngressTlsConfig>,
    pub(crate) max_message_bytes: usize,
    pub(crate) max_concurrent_streams: u32,
    pub(crate) allow_unauthenticated: bool,
    pub(crate) compression: CompressionMode,
    pub(crate) enable_reflection: bool,
    pub(crate) keepalive_interval_secs: Option<u64>,
    pub(crate) keepalive_timeout_secs: u64,
}
