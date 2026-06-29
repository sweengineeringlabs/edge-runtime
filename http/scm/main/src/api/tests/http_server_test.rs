//! Contract tests for the [`HttpServer`] default-method surface.

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use futures::future::BoxFuture;
    use swe_edge_ingress_http::{
        HttpHealthCheck, HttpIngress, HttpIngressResult, HttpRequest, HttpResponse,
    };
    use tokio::net::TcpListener;

    use crate::api::{AxumHttpServer, HttpServer, HttpServerError, HttpServerSvc};

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

    struct TestServer(AxumHttpServer);
    impl HttpServer for TestServer {
        fn serve<'s>(&'s self) -> BoxFuture<'s, Result<(), HttpServerError>> {
            Box::pin(async { Ok(()) })
        }
        fn serve_with_shutdown<'s>(
            &'s self,
            _shutdown: BoxFuture<'static, ()>,
        ) -> BoxFuture<'s, Result<(), HttpServerError>> {
            Box::pin(async { Ok(()) })
        }
    }

    fn ingress() -> Arc<dyn HttpIngress> {
        Arc::new(NoopIngress)
    }

    /// @covers: request_timeout
    #[test]
    fn test_request_timeout_default_is_thirty_seconds_happy() {
        let s = TestServer(AxumHttpServer::new("0.0.0.0:0", ingress()));
        assert_eq!(s.request_timeout().as_secs(), 30);
    }

    /// @covers: axum_helper
    #[test]
    fn test_axum_helper_returns_zero_sized_type_happy() {
        let s = TestServer(AxumHttpServer::new("0.0.0.0:0", ingress()));
        let h = s.axum_helper();
        assert_eq!(
            std::mem::size_of_val(&h),
            0,
            "AxumHttpServerHelper must be zero-sized"
        );
    }

    /// @covers: new_server
    #[test]
    fn test_new_server_constructs_with_correct_bind_happy() {
        let srv = TestServer::new_server("0.0.0.0:1234".to_string(), ingress());
        assert_eq!(srv.bind, "0.0.0.0:1234");
    }

    /// @covers: new_server_svc
    #[test]
    fn test_new_server_svc_returns_zero_sized_factory_happy() {
        let svc: HttpServerSvc = TestServer::new_server_svc();
        assert_eq!(
            std::mem::size_of_val(&svc),
            0,
            "HttpServerSvc must be zero-sized"
        );
    }

    /// @covers: serve_with_listener
    #[test]
    fn test_serve_with_listener_default_returns_not_implemented_error() {
        let s = TestServer(AxumHttpServer::new("0.0.0.0:0", ingress()));
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        let result = rt.block_on(async {
            let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
            s.serve_with_listener(listener, Box::pin(async {})).await
        });
        assert!(result.is_err(), "default impl must return an error");
    }

    /// @covers: builder_bind
    #[test]
    fn test_builder_bind_returns_configured_address_edge() {
        use crate::api::AxumHttpServerBuilder;
        let s = TestServer(AxumHttpServer::new("0.0.0.0:0", ingress()));
        let builder = AxumHttpServerBuilder::new("127.0.0.1:8080", ingress());
        assert_eq!(s.builder_bind(&builder), "127.0.0.1:8080");
    }
}
