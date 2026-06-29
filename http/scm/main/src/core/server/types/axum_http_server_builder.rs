//! Inherent constructors and builder methods for [`crate::api::AxumHttpServerBuilder`].

use std::sync::Arc;
use std::time::Duration;

use edge_domain_security::PemTlsConfig;
use swe_edge_ingress_http::{HttpIngress, DEFAULT_REQUEST_TIMEOUT, MAX_BODY_BYTES};
use swe_edge_ingress_verifier::TokenVerifier;

use crate::api::{AxumHttpServer, AxumHttpServerBuilder};

impl AxumHttpServerBuilder {
    /// Creates a builder bound to `bind` delegating requests to `handler`.
    pub fn new(bind: impl Into<String>, handler: Arc<dyn HttpIngress>) -> Self {
        Self {
            bind: bind.into(),
            handler,
            body_limit: MAX_BODY_BYTES,
            request_timeout: DEFAULT_REQUEST_TIMEOUT,
            tls: None,
            bearer_verifier: None,
        }
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

    /// Consume the builder and return a configured [`AxumHttpServer`].
    pub fn build(self) -> AxumHttpServer {
        let s = AxumHttpServer::new(self.bind, self.handler)
            .with_body_limit(self.body_limit)
            .with_request_timeout(self.request_timeout);
        let s = Self::apply_tls(s, self.tls);
        Self::apply_auth(s, self.bearer_verifier)
    }

    fn apply_tls(s: AxumHttpServer, tls: Option<PemTlsConfig>) -> AxumHttpServer {
        match tls {
            Some(cfg) => s.with_tls(cfg),
            None => s,
        }
    }

    fn apply_auth(s: AxumHttpServer, v: Option<Arc<dyn TokenVerifier>>) -> AxumHttpServer {
        match v {
            Some(verifier) => s.with_bearer_auth(verifier),
            None => s,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use futures::future::BoxFuture;
    use swe_edge_ingress_http::{
        HttpHealthCheck, HttpIngress, HttpIngressResult, HttpRequest, HttpResponse,
    };
    use swe_edge_ingress_verifier::{Claims, TokenVerifier, VerifierError};

    use super::*;

    struct AxumHttpServerBuilderNoopIngress;
    impl HttpIngress for AxumHttpServerBuilderNoopIngress {
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

    struct AxumHttpServerBuilderDenyAllVerifier;
    impl TokenVerifier for AxumHttpServerBuilderDenyAllVerifier {
        fn verify(&self, _: &str) -> Result<Claims, VerifierError> {
            Err(VerifierError::Expired)
        }
    }

    fn noop() -> Arc<dyn HttpIngress> {
        Arc::new(AxumHttpServerBuilderNoopIngress)
    }

    fn base() -> AxumHttpServer {
        AxumHttpServer::new("0.0.0.0:0", noop())
    }

    /// @covers: apply_tls
    #[test]
    fn test_apply_tls_none_returns_server_unchanged() {
        let s = base();
        let s2 = AxumHttpServerBuilder::apply_tls(s, None);
        assert!(s2.tls.is_none(), "None tls must leave server unchanged");
    }

    /// @covers: apply_tls
    #[test]
    fn test_apply_tls_some_sets_cert_path() {
        let s = base();
        let s2 = AxumHttpServerBuilder::apply_tls(
            s,
            Some(edge_domain_security::PemTlsConfig::tls("c.pem", "k.pem")),
        );
        assert_eq!(
            s2.tls.as_ref().map(|t| t.cert_pem_path.as_str()),
            Some("c.pem")
        );
    }

    /// @covers: apply_auth
    #[test]
    fn test_apply_auth_none_leaves_no_verifier() {
        let s = base();
        let s2 = AxumHttpServerBuilder::apply_auth(s, None);
        assert!(
            s2.bearer_verifier.is_none(),
            "None verifier must leave server unchanged"
        );
    }

    /// @covers: apply_auth
    #[test]
    fn test_apply_auth_some_sets_verifier() {
        let without = base();
        assert!(
            without.bearer_verifier.is_none(),
            "baseline must have no verifier"
        );
        let with_auth = AxumHttpServerBuilder::apply_auth(
            base(),
            Some(Arc::new(AxumHttpServerBuilderDenyAllVerifier)),
        );
        assert!(
            with_auth.bearer_verifier.is_some(),
            "Some verifier must be set"
        );
    }

    /// @covers: new
    #[test]
    fn test_new_stores_bind() {
        let b = AxumHttpServerBuilder::new("0.0.0.0:5000", noop());
        assert_eq!(b.bind, "0.0.0.0:5000");
    }

    /// @covers: with_body_limit
    #[test]
    fn test_with_body_limit_stores_value() {
        let b = AxumHttpServerBuilder::new("0.0.0.0:0", noop()).with_body_limit(2048);
        assert_eq!(b.body_limit, 2048);
    }

    /// @covers: with_request_timeout
    #[test]
    fn test_with_request_timeout_stores_value() {
        use std::time::Duration;
        let b = AxumHttpServerBuilder::new("0.0.0.0:0", noop())
            .with_request_timeout(Duration::from_secs(20));
        assert_eq!(b.request_timeout, Duration::from_secs(20));
    }

    /// @covers: with_tls
    #[test]
    fn test_with_tls_stores_cert_path() {
        let b = AxumHttpServerBuilder::new("0.0.0.0:0", noop())
            .with_tls(edge_domain_security::PemTlsConfig::tls("srv.pem", "k.pem"));
        assert_eq!(
            b.tls.as_ref().map(|t| t.cert_pem_path.as_str()),
            Some("srv.pem")
        );
    }

    /// @covers: with_bearer_auth
    #[test]
    fn test_with_bearer_auth_sets_verifier() {
        let b_no_auth = AxumHttpServerBuilder::new("0.0.0.0:0", noop());
        assert!(
            b_no_auth.bearer_verifier.is_none(),
            "fresh builder must have no verifier"
        );
        let b = b_no_auth.with_bearer_auth(Arc::new(AxumHttpServerBuilderDenyAllVerifier));
        assert!(b.bearer_verifier.is_some());
    }

    /// @covers: build
    #[test]
    fn test_build_preserves_bind() {
        let s = AxumHttpServerBuilder::new("0.0.0.0:6000", noop()).build();
        assert_eq!(s.bind, "0.0.0.0:6000");
    }
}
