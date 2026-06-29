//! Colocated tests for [`crate::api::HttpServerSvc`] factory methods.

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use futures::future::BoxFuture;
    use swe_edge_ingress_http::{
        HttpHealthCheck, HttpIngress, HttpIngressResult, HttpRequest, HttpResponse,
    };

    use crate::api::HttpServerSvc;

    struct NoopIngress;
    impl HttpIngress for NoopIngress {
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

    fn ingress() -> Arc<dyn HttpIngress> {
        Arc::new(NoopIngress)
    }

    /// @covers: new_server
    #[test]
    fn test_new_server_sets_bind_address_happy() {
        let srv = HttpServerSvc::new_server("0.0.0.0:8080".to_string(), ingress());
        assert_eq!(srv.bind, "0.0.0.0:8080");
    }

    /// @covers: builder
    #[test]
    fn test_builder_returns_builder_with_correct_bind_happy() {
        let b = HttpServerSvc::builder("0.0.0.0:9090".to_string(), ingress());
        assert_eq!(b.bind, "0.0.0.0:9090");
    }

    /// @covers: new_server
    #[test]
    fn test_new_server_and_builder_produce_same_bind_edge() {
        let addr = "0.0.0.0:7070".to_string();
        let srv = HttpServerSvc::new_server(addr.clone(), ingress());
        let built = HttpServerSvc::builder(addr.clone(), ingress()).build();
        assert_eq!(srv.bind, built.bind);
    }
}
