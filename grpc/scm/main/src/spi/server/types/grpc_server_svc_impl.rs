//! GrpcServerSvcOps trait impl for GrpcServerSvc.
use std::net::SocketAddr;

use crate::api::{GrpcServerConfigBuild, GrpcServerConfigBuilder, GrpcServerSvc, GrpcServerSvcOps};

impl GrpcServerSvcOps for GrpcServerSvc {
    fn create_config_builder(bind: SocketAddr) -> GrpcServerConfigBuilder {
        GrpcServerConfigBuilder::new(bind)
    }
}
