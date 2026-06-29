//! Inherent factory methods for [`crate::api::HttpServerSvc`].

use std::sync::Arc;

use swe_edge_ingress_http::HttpIngress;

use crate::api::{AxumHttpServer, AxumHttpServerBuilder, HttpServerSvc};

impl HttpServerSvc {
    /// Construct a new [`AxumHttpServer`] bound to `bind`, delegating all
    /// inbound requests to `handler`.
    pub fn new_server(bind: String, handler: Arc<dyn HttpIngress>) -> AxumHttpServer {
        AxumHttpServer::new(Self::normalize_bind(bind), handler)
    }

    /// Return a fluent builder for constructing an [`AxumHttpServer`].
    pub fn builder(bind: String, handler: Arc<dyn HttpIngress>) -> AxumHttpServerBuilder {
        AxumHttpServerBuilder::new(Self::normalize_bind(bind), handler)
    }

    fn normalize_bind(bind: String) -> String {
        bind.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use futures::future::BoxFuture;
    use swe_edge_ingress_http::{
        HttpHealthCheck, HttpIngress, HttpIngressResult, HttpRequest, HttpResponse,
    };

    use super::*;

    struct HttpServerSvcNoopIngress;
    impl HttpIngress for HttpServerSvcNoopIngress {
        fn handle(
            &self,
            _: HttpRequest,
            _: edge_domain::SecurityContext,
        ) -> BoxFuture<'_, HttpIngressResult<HttpResponse>> {
            Box::pin(async { Ok(HttpResponse::new(200, vec![])) })
        }
        fn health_check(&self) -> BoxFuture<'_, HttpIngressResult<HttpHealthCheck>> {
            Box::pin(async { Ok(HttpHealthCheck::healthy()) })
        }
    }

    fn noop() -> Arc<dyn HttpIngress> {
        Arc::new(HttpServerSvcNoopIngress)
    }

    /// @covers: normalize_bind
    #[test]
    fn test_normalize_bind_strips_whitespace() {
        assert_eq!(
            HttpServerSvc::normalize_bind("  0.0.0.0:8080  ".to_string()),
            "0.0.0.0:8080"
        );
    }

    /// @covers: normalize_bind
    #[test]
    fn test_normalize_bind_leaves_clean_address_unchanged() {
        let addr = "127.0.0.1:9090".to_string();
        assert_eq!(HttpServerSvc::normalize_bind(addr.clone()), addr);
    }

    /// @covers: new_server
    #[test]
    fn test_new_server_stores_bind() {
        let s = HttpServerSvc::new_server("0.0.0.0:3000".to_string(), noop());
        assert_eq!(s.bind, "0.0.0.0:3000");
    }

    /// @covers: new_server
    #[test]
    fn test_new_server_normalizes_whitespace_in_bind() {
        let s = HttpServerSvc::new_server("  0.0.0.0:3001  ".to_string(), noop());
        assert_eq!(
            s.bind, "0.0.0.0:3001",
            "new_server must strip whitespace from bind"
        );
    }

    /// @covers: builder
    #[test]
    fn test_builder_stores_bind() {
        let b = HttpServerSvc::builder("0.0.0.0:4000".to_string(), noop());
        assert_eq!(b.bind, "0.0.0.0:4000");
    }

    /// @covers: builder
    #[test]
    fn test_builder_normalizes_whitespace_in_bind() {
        let b = HttpServerSvc::builder("  127.0.0.1:4001  ".to_string(), noop());
        assert_eq!(
            b.bind, "127.0.0.1:4001",
            "builder must strip whitespace from bind"
        );
    }
}
