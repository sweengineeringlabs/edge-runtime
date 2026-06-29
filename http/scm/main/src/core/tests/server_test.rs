//! Colocated tests for [`crate::api::AxumHttpServer`] constructors.

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use futures::future::BoxFuture;
    use swe_edge_ingress_http::{
        HttpHealthCheck, HttpIngress, HttpIngressResult, HttpRequest, HttpResponse,
    };

    use crate::api::AxumHttpServer;

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

    /// @covers: new
    #[test]
    fn test_new_sets_bind_address_happy() {
        let srv = AxumHttpServer::new("0.0.0.0:8080", ingress());
        assert_eq!(srv.bind, "0.0.0.0:8080");
    }

    /// @covers: with_body_limit
    #[test]
    fn test_with_body_limit_overrides_default_happy() {
        let srv = AxumHttpServer::new("0.0.0.0:0", ingress()).with_body_limit(1024);
        assert_eq!(srv.body_limit, 1024);
    }

    /// @covers: with_request_timeout
    #[test]
    fn test_with_request_timeout_overrides_default_happy() {
        use std::time::Duration;
        let srv = AxumHttpServer::new("0.0.0.0:0", ingress())
            .with_request_timeout(Duration::from_secs(5));
        assert_eq!(srv.request_timeout, Duration::from_secs(5));
    }

    /// @covers: with_tls
    #[test]
    fn test_with_tls_sets_cert_path_happy() {
        use edge_domain_security::PemTlsConfig;
        let srv = AxumHttpServer::new("0.0.0.0:0", ingress())
            .with_tls(PemTlsConfig::tls("cert.pem", "key.pem"));
        assert_eq!(
            srv.tls.as_ref().map(|t| t.cert_pem_path.as_str()),
            Some("cert.pem")
        );
    }

    /// @covers: with_stream_handler
    #[test]
    fn test_with_stream_handler_sets_handler_edge() {
        use swe_edge_ingress_http::{HttpIngressResult, HttpStream, SseStream, WsChannel};

        struct NoopStream;
        impl HttpStream for NoopStream {
            fn handle_sse(
                &self,
                _: HttpRequest,
                _: edge_domain::SecurityContext,
            ) -> BoxFuture<'_, HttpIngressResult<SseStream>> {
                Box::pin(async {
                    Err(swe_edge_ingress_http::HttpIngressError::MethodNotAllowed(
                        "".into(),
                    ))
                })
            }
            fn handle_websocket(
                &self,
                _: HttpRequest,
                _: edge_domain::SecurityContext,
                _: WsChannel,
            ) -> BoxFuture<'_, HttpIngressResult<()>> {
                Box::pin(async { Ok(()) })
            }
        }

        let fresh = AxumHttpServer::new("0.0.0.0:0", ingress());
        assert!(
            fresh.stream_handler.is_none(),
            "fresh server must have no stream handler"
        );
        let srv = fresh.with_stream_handler(Arc::new(NoopStream));
        assert!(
            srv.stream_handler.is_some(),
            "with_stream_handler must set the handler"
        );
    }

    /// @covers: with_bearer_auth
    #[test]
    fn test_with_bearer_auth_sets_verifier_edge() {
        use swe_edge_ingress_verifier::{Claims, TokenVerifier, VerifierError};

        struct NoopVerifier;
        impl TokenVerifier for NoopVerifier {
            fn verify(&self, _token: &str) -> Result<Claims, VerifierError> {
                Err(VerifierError::Expired)
            }
        }

        let fresh = AxumHttpServer::new("0.0.0.0:0", ingress());
        assert!(
            fresh.bearer_verifier.is_none(),
            "fresh server must have no bearer verifier"
        );
        let srv = fresh.with_bearer_auth(Arc::new(NoopVerifier));
        assert!(
            srv.bearer_verifier.is_some(),
            "with_bearer_auth must set the verifier"
        );
    }
}
