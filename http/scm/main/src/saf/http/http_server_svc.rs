//! SAF factory surface for [`HttpServer`].
//!
//! Implements factory methods on [`HttpServerSvc`] (declared in `api/`).

use std::sync::Arc;

use swe_edge_ingress_http::HttpIngress;

use crate::api::{AxumHttpServer, AxumHttpServerBuilder};

pub use crate::api::HttpServerSvc;

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
