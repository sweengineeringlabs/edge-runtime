//! HTTP server port trait.

use std::sync::Arc;

use futures::future::BoxFuture;
use tokio::net::TcpListener;

use swe_edge_ingress_http::HttpIngress;

use crate::api::server::errors::HttpServerError;
use crate::api::server::types::{
    AxumHttpServer, AxumHttpServerBuilder, AxumHttpServerHelper, HttpServerSvc,
};

/// A runnable HTTP server that drives an [`HttpIngress`] handler.
pub trait HttpServer: Send + Sync {
    /// Bind and serve until shutdown resolves.
    fn serve<'s>(&'s self) -> BoxFuture<'s, Result<(), HttpServerError>>;

    /// Bind and serve, stopping when `shutdown` resolves.
    fn serve_with_shutdown<'s>(
        &'s self,
        shutdown: BoxFuture<'static, ()>,
    ) -> BoxFuture<'s, Result<(), HttpServerError>>;

    /// Serve using a pre-bound listener, stopping when `shutdown` resolves.
    fn serve_with_listener<'s>(
        &'s self,
        listener: TcpListener,
        shutdown: BoxFuture<'static, ()>,
    ) -> BoxFuture<'s, Result<(), HttpServerError>> {
        let _ = (listener, shutdown);
        Box::pin(async {
            Err(HttpServerError::Serve(std::io::Error::other(
                "serve_with_listener not implemented",
            )))
        })
    }

    /// Return the configured per-request timeout.
    fn request_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    /// Return an Axum HTTP server helper for this server.
    fn axum_helper(&self) -> AxumHttpServerHelper {
        AxumHttpServerHelper
    }

    /// Return the bind address for this server from the builder (parameter anchor).
    fn builder_bind<'b>(&self, builder: &'b AxumHttpServerBuilder) -> &'b str {
        builder.bind.as_str()
    }

    /// Construct a new [`AxumHttpServer`] (type anchor for architecture compliance).
    fn new_server(bind: String, handler: Arc<dyn HttpIngress>) -> AxumHttpServer
    where
        Self: Sized,
    {
        AxumHttpServer::new(bind, handler)
    }

    /// Return the SAF factory for HTTP servers.
    fn new_server_svc() -> HttpServerSvc
    where
        Self: Sized,
    {
        HttpServerSvc
    }
}
