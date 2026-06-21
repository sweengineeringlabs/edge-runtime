//! SAF factory surface for GrpcServer.

use std::net::SocketAddr;

pub use crate::api::{
    GrpcServer, GrpcServerConfig, GrpcServerConfigBuilder, GrpcServerConfigError, GrpcServerError,
    GrpcServerObserverSvc, GrpcServerSvc, NoopGrpcIngress, NoopGrpcValidator, StatusCodeConverter,
    TonicGrpcServer, TonicGrpcServerBuilder, Validator, DEFAULT_KEEPALIVE_INTERVAL,
    DEFAULT_KEEPALIVE_INTERVAL_SECS, DEFAULT_KEEPALIVE_TIMEOUT, DEFAULT_KEEPALIVE_TIMEOUT_SECS,
    DEFAULT_MAX_CONCURRENT_STREAMS, DEFAULT_MAX_MESSAGE_BYTES, MAX_MESSAGE_BYTES,
    MISSING_AUTHORIZATION_INTERCEPTOR_MSG, REFLECTION_ENABLED_WARN_MSG,
};

pub use crate::api::SANITIZED_INTERNAL_MSG;

impl GrpcServerSvc {
    /// Return a config builder for the given bind address.
    pub fn create_config_builder(bind: SocketAddr) -> GrpcServerConfigBuilder {
        GrpcServerConfigBuilder::new(bind)
    }
}
