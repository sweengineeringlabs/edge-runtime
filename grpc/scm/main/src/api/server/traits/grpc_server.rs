//! A runnable gRPC server trait.

use futures::future::BoxFuture;

use crate::api::server::error::GrpcServerError;

/// A runnable gRPC server that drives a [`GrpcIngress`](swe_edge_ingress_grpc::GrpcIngress) handler.
pub trait GrpcServer: Send + Sync {
    /// Bind and serve until `shutdown` resolves.
    fn serve<'s>(&'s self) -> BoxFuture<'s, Result<(), GrpcServerError>>;
}
