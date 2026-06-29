//! Inherent constructors and builder methods for [`crate::api::AxumHttpServer`].

use std::sync::Arc;
use std::time::Duration;

use edge_domain_security::PemTlsConfig;
use swe_edge_ingress_http::{HttpIngress, HttpStream, DEFAULT_REQUEST_TIMEOUT, MAX_BODY_BYTES};
use swe_edge_ingress_verifier::TokenVerifier;

use crate::api::AxumHttpServer;

impl AxumHttpServer {
    /// Create a server binding to `bind` and delegating requests to `handler`.
    pub fn new(bind: impl Into<String>, handler: Arc<dyn HttpIngress>) -> Self {
        Self::init(bind.into(), handler)
    }

    /// Attach an [`HttpStream`] handler for SSE and WebSocket requests.
    pub fn with_stream_handler(mut self, handler: Arc<dyn HttpStream>) -> Self {
        self.stream_handler = Some(handler);
        self
    }

    /// Override the maximum request body size (default: [`MAX_BODY_BYTES`]).
    pub fn with_body_limit(mut self, limit: usize) -> Self {
        self.body_limit = limit;
        self
    }

    /// Override the per-request timeout (default: [`DEFAULT_REQUEST_TIMEOUT`]).
    pub fn with_request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = timeout;
        self
    }

    /// Enable TLS or mTLS.
    pub fn with_tls(mut self, config: PemTlsConfig) -> Self {
        self.tls = Some(config);
        self
    }

    /// Enable JWT bearer authentication.
    pub fn with_bearer_auth(mut self, verifier: Arc<dyn TokenVerifier>) -> Self {
        self.bearer_verifier = Some(verifier);
        self
    }

    fn init(bind: String, handler: Arc<dyn HttpIngress>) -> Self {
        Self {
            bind,
            handler,
            body_limit: MAX_BODY_BYTES,
            request_timeout: DEFAULT_REQUEST_TIMEOUT,
            tls: None,
            bearer_verifier: None,
            stream_handler: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use futures::future::BoxFuture;
    use swe_edge_ingress_http::{
        HttpHealthCheck, HttpIngress, HttpIngressResult, HttpRequest, HttpResponse,
        DEFAULT_REQUEST_TIMEOUT, MAX_BODY_BYTES,
    };

    use super::*;

    struct AxumHttpServerNoopIngress;
    impl HttpIngress for AxumHttpServerNoopIngress {
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
        Arc::new(AxumHttpServerNoopIngress)
    }

    /// @covers: init
    #[test]
    fn test_init_sets_expected_defaults() {
        let s = AxumHttpServer::init("0.0.0.0:0".to_string(), noop());
        assert_eq!(s.bind, "0.0.0.0:0");
        assert_eq!(
            s.body_limit, MAX_BODY_BYTES,
            "default body limit must be MAX_BODY_BYTES"
        );
        assert_eq!(
            s.request_timeout, DEFAULT_REQUEST_TIMEOUT,
            "default timeout must be DEFAULT_REQUEST_TIMEOUT"
        );
        assert!(s.tls.is_none(), "tls must be None by default");
        assert!(
            s.bearer_verifier.is_none(),
            "bearer_verifier must be None by default"
        );
        assert!(
            s.stream_handler.is_none(),
            "stream_handler must be None by default"
        );
    }

    /// @covers: init
    #[test]
    fn test_init_stores_bind_unchanged() {
        let addr = "127.0.0.1:9999".to_string();
        let s = AxumHttpServer::init(addr.clone(), noop());
        assert_eq!(s.bind, addr);
    }

    /// @covers: new
    #[test]
    fn test_new_stores_bind() {
        let s = AxumHttpServer::new("0.0.0.0:1234", noop());
        assert_eq!(s.bind, "0.0.0.0:1234");
    }

    /// @covers: with_body_limit
    #[test]
    fn test_with_body_limit_stores_value() {
        let s = AxumHttpServer::new("0.0.0.0:0", noop()).with_body_limit(1024);
        assert_eq!(s.body_limit, 1024);
    }

    /// @covers: with_request_timeout
    #[test]
    fn test_with_request_timeout_stores_value() {
        let s =
            AxumHttpServer::new("0.0.0.0:0", noop()).with_request_timeout(Duration::from_secs(5));
        assert_eq!(s.request_timeout, Duration::from_secs(5));
    }

    /// @covers: with_tls
    #[test]
    fn test_with_tls_stores_cert_path() {
        let s = AxumHttpServer::new("0.0.0.0:0", noop())
            .with_tls(edge_domain_security::PemTlsConfig::tls("c.pem", "k.pem"));
        assert_eq!(
            s.tls.as_ref().map(|t| t.cert_pem_path.as_str()),
            Some("c.pem")
        );
    }

    /// @covers: with_bearer_auth
    #[test]
    fn test_with_bearer_auth_sets_verifier() {
        use swe_edge_ingress_verifier::{Claims, TokenVerifier, VerifierError};

        struct AxumHttpServerDenyAllVerifier;
        impl TokenVerifier for AxumHttpServerDenyAllVerifier {
            fn verify(&self, _: &str) -> Result<Claims, VerifierError> {
                Err(VerifierError::Expired)
            }
        }

        let fresh = AxumHttpServer::new("0.0.0.0:0", noop());
        assert!(
            fresh.bearer_verifier.is_none(),
            "fresh server must have no verifier"
        );
        let s = fresh.with_bearer_auth(Arc::new(AxumHttpServerDenyAllVerifier));
        assert!(s.bearer_verifier.is_some());
    }

    /// @covers: with_stream_handler
    #[test]
    fn test_with_stream_handler_sets_handler() {
        use futures::future::BoxFuture;
        use swe_edge_ingress_http::{HttpIngressResult, HttpStream, SseStream, WsChannel};

        struct AxumHttpServerNoopStream;
        impl HttpStream for AxumHttpServerNoopStream {
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

        let fresh = AxumHttpServer::new("0.0.0.0:0", noop());
        assert!(
            fresh.stream_handler.is_none(),
            "fresh server must have no stream handler"
        );
        let s = fresh.with_stream_handler(Arc::new(AxumHttpServerNoopStream));
        assert!(s.stream_handler.is_some());
    }
}
