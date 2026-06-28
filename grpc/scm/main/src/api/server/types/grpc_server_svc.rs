//! GrpcServerSvc type declaration and inherent impl.

use std::net::SocketAddr;

use crate::api::GrpcServerConfigBuilder;

/// Factory for gRPC server objects.
pub struct GrpcServerSvc;

impl GrpcServerSvc {
    /// Return a config builder for the given bind address.
    pub fn create_config_builder(bind: SocketAddr) -> GrpcServerConfigBuilder {
        GrpcServerConfigBuilder::new(bind)
    }
}
