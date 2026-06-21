//! SAF factory surface for [`HttpServer`].
//!
//! Provides the `HttpServerSvc` factory for constructing HTTP servers without
//! exposing the concrete Axum implementation to consumers.

use std::sync::Arc;

use swe_edge_ingress_http::HttpIngress;

use crate::api::{AxumHttpServer, AxumHttpServerBuilder};

/// Factory for HTTP server construction.
///
/// Consumers call [`HttpServerSvc::new_server`] to obtain an [`AxumHttpServer`]
/// without naming the concrete type directly.
pub(crate) struct HttpServerSvc;

impl HttpServerSvc {
    /// Construct a new [`AxumHttpServer`] bound to `bind`, delegating all
    /// inbound requests to `handler`.
    pub fn new_server(bind: String, handler: Arc<dyn HttpIngress>) -> AxumHttpServer {
        AxumHttpServer::new(bind, handler)
    }

    /// Return a fluent builder for constructing an [`AxumHttpServer`].
    pub fn builder(bind: String, handler: Arc<dyn HttpIngress>) -> AxumHttpServerBuilder {
        AxumHttpServerBuilder::new(bind, handler)
    }
}
