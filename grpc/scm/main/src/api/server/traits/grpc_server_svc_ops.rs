//! Operations performed via a gRPC server service object.

use std::net::SocketAddr;

use crate::api::GrpcServerConfigBuilder;

/// Operations performed via [`GrpcServerSvc`] as a service object.
///
/// Static methods are gated on `where Self: Sized`; [`GrpcServerSvcOps::svc_marker`]
/// keeps the trait object-safe.
pub trait GrpcServerSvcOps {
    /// Return a config builder pre-seeded with `bind`.
    fn create_config_builder(bind: SocketAddr) -> GrpcServerConfigBuilder
    where
        Self: Sized;
    /// Object-safe marker method.
    fn svc_marker(&self) -> bool {
        true
    }
}
