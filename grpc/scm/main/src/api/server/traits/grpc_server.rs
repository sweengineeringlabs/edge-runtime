//! A runnable gRPC server trait.

use std::net::SocketAddr;
use std::sync::Arc;

use futures::future::BoxFuture;
use swe_edge_ingress_grpc::GrpcIngress;

use crate::api::server::errors::GrpcServerConfigError;
use crate::api::server::errors::GrpcServerError;
use crate::api::server::types::{
    GrpcServerConfig, GrpcServerConfigBuilder, GrpcServerObserverSvc, GrpcServerSvc,
    StatusCodeConverter, TonicGrpcServer, TonicGrpcServerBuilder,
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
        TonicGrpcServer::new(bind, handler)
    }

    /// Construct a [`TonicGrpcServer`] from a [`GrpcServerConfig`], or return a [`GrpcServerConfigError`].
    fn from_config(
        config: &GrpcServerConfig,
        handler: Arc<dyn GrpcIngress>,
    ) -> Result<TonicGrpcServer, GrpcServerConfigError>
    where
        Self: Sized,
    {
        TonicGrpcServer::from_config(config, handler)
    }

    /// Construct a default [`GrpcServerConfigBuilder`] (type anchor).
    fn new_config_builder(bind: SocketAddr) -> GrpcServerConfigBuilder
    where
        Self: Sized,
    {
        GrpcServerConfigBuilder::new(bind)
    }

    /// Return a [`GrpcServerSvc`] factory (type anchor).
    fn new_server_svc() -> GrpcServerSvc
    where
        Self: Sized,
    {
        GrpcServerSvc
    }

    /// Return a [`GrpcServerObserverSvc`] factory (type anchor).
    fn new_observer_svc() -> GrpcServerObserverSvc
    where
        Self: Sized,
    {
        GrpcServerObserverSvc
    }

    /// Convert a status code via [`StatusCodeConverter`] (type anchor).
    fn status_converter() -> StatusCodeConverter
    where
        Self: Sized,
    {
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
