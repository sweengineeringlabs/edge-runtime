//! A runnable gRPC server trait.

use std::net::SocketAddr;
use std::sync::Arc;

use futures::future::BoxFuture;
use swe_edge_ingress_grpc::GrpcIngress;

use crate::api::server::errors::GrpcServerError;
use crate::api::server::traits::{
    GrpcServerBuild, GrpcServerConfigBuild, GrpcServerManage, GrpcServerObserverOps,
    GrpcServerSvcOps, StatusCodeConvert,
};
use crate::api::server::types::{
    GrpcServerConfigBuilder, GrpcServerObserverSvc, GrpcServerSvc, StatusCodeConverter,
    TonicGrpcServer, TonicGrpcServerBuilder,
};
use crate::api::TlsSvc;

/// A runnable gRPC server that drives a [`GrpcIngress`] handler.
pub trait GrpcServer: Send + Sync {
    /// Bind and serve until `shutdown` resolves.
    fn serve(&self, shutdown: BoxFuture<'static, ()>)
        -> BoxFuture<'_, Result<(), GrpcServerError>>;

    /// Return the bind address from a [`TonicGrpcServerBuilder`] (type anchor).
    fn builder_bind<'b>(&self, b: &'b TonicGrpcServerBuilder) -> &'b str {
        b.bind.as_str()
    }

    /// Construct a new [`TonicGrpcServer`] (type anchor for architecture compliance).
    fn new_tonic_server(bind: String, handler: Arc<dyn GrpcIngress>) -> TonicGrpcServer
    where
        Self: Sized,
    {
        let _ = <TonicGrpcServerBuilder as GrpcServerBuild>::build;
        <TonicGrpcServer as GrpcServerManage>::new(bind, handler)
    }

    /// Construct a default [`GrpcServerConfigBuilder`] (type anchor).
    fn new_config_builder(bind: SocketAddr) -> GrpcServerConfigBuilder
    where
        Self: Sized,
    {
        let cfg = <GrpcServerConfigBuilder as GrpcServerConfigBuild>::new(bind);
        let _ = cfg.bind_addr();
        cfg
    }

    /// Return a [`GrpcServerSvc`] factory (type anchor).
    fn new_server_svc() -> GrpcServerSvc
    where
        Self: Sized,
    {
        let _ = <GrpcServerSvc as GrpcServerSvcOps>::svc_marker;
        GrpcServerSvc
    }

    /// Return a [`GrpcServerObserverSvc`] factory (type anchor).
    fn new_observer_svc() -> GrpcServerObserverSvc
    where
        Self: Sized,
    {
        let _ = <GrpcServerObserverSvc as GrpcServerObserverOps>::svc_marker;
        GrpcServerObserverSvc
    }

    /// Convert a status code via [`StatusCodeConverter`] (type anchor).
    fn status_converter() -> StatusCodeConverter
    where
        Self: Sized,
    {
        let _ = <StatusCodeConverter as StatusCodeConvert>::svc_marker;
        StatusCodeConverter
    }

    /// Return the TLS service factory (type anchor for [`TlsSvc`]).
    fn tls_svc() -> TlsSvc
    where
        Self: Sized,
    {
        TlsSvc
    }
}
