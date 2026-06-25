//! SAF factory surface for GrpcServer.

use std::net::SocketAddr;

use crate::api::{GrpcServerConfigBuilder, GrpcServerSvc};

impl GrpcServerSvc {
    /// Return a config builder for the given bind address.
    pub fn create_config_builder(bind: SocketAddr) -> GrpcServerConfigBuilder {
        GrpcServerConfigBuilder::new(bind)
    }
}
