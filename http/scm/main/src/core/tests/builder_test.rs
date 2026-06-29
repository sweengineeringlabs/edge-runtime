//! Colocated tests for [`crate::api::AxumHttpServerBuilder`] constructors.

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use futures::future::BoxFuture;
    use swe_edge_ingress_http::{
        HttpHealthCheck, HttpIngress, HttpIngressResult, HttpRequest, HttpResponse,
    };

    use crate::api::AxumHttpServerBuilder;

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
        let b = AxumHttpServerBuilder::new("0.0.0.0:8080", ingress());
        assert_eq!(b.bind, "0.0.0.0:8080");
    }

    /// @covers: with_body_limit
    #[test]
    fn test_with_body_limit_overrides_default_happy() {
        let b = AxumHttpServerBuilder::new("0.0.0.0:0", ingress()).with_body_limit(512);
        assert_eq!(b.body_limit, 512);
    }

    /// @covers: with_request_timeout
    #[test]
    fn test_with_request_timeout_overrides_default_happy() {
        use std::time::Duration;
        let b = AxumHttpServerBuilder::new("0.0.0.0:0", ingress())
            .with_request_timeout(Duration::from_secs(10));
        assert_eq!(b.request_timeout, Duration::from_secs(10));
    }

    /// @covers: with_tls
    #[test]
    fn test_with_tls_sets_cert_path_happy() {
        use edge_domain_security::PemTlsConfig;
        let b = AxumHttpServerBuilder::new("0.0.0.0:0", ingress())
            .with_tls(PemTlsConfig::tls("c.pem", "k.pem"));
        assert_eq!(
            b.tls.as_ref().map(|t| t.cert_pem_path.as_str()),
            Some("c.pem")
        );
    }

    /// @covers: build
    #[test]
    fn test_build_produces_server_with_correct_bind_happy() {
        let srv = AxumHttpServerBuilder::new("0.0.0.0:9090", ingress()).build();
        assert_eq!(srv.bind, "0.0.0.0:9090");
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

        let b_no_auth = AxumHttpServerBuilder::new("0.0.0.0:0", ingress());
        assert!(
            b_no_auth.bearer_verifier.is_none(),
            "fresh builder must have no verifier"
        );
        let b = b_no_auth.with_bearer_auth(Arc::new(NoopVerifier));
        assert!(
            b.bearer_verifier.is_some(),
            "with_bearer_auth must set the verifier"
        );
    }
}
