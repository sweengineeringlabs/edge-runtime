//! JWT principal for authenticated HTTP request contexts.

use std::sync::Arc;

use axum::http::{header, HeaderValue, StatusCode};
use edge_domain::{Principal, SecurityContext};
use swe_edge_ingress_verifier::TokenVerifier;

use crate::api::AxumHttpServerHelper;

pub(super) struct JwtPrincipal {
    pub(super) sub: String,
}

impl Principal for JwtPrincipal {
    fn id(&self) -> &str {
        &self.sub
    }

    fn kind(&self) -> &str {
        const KIND: &str = "jwt";
        KIND
    }
}

impl AxumHttpServerHelper {
    /// Verify bearer token and attach a [`SecurityContext`] to the request.
    ///
    /// Returns the enriched request on success, or a `401` response on failure.
    /// If no verifier is configured the request passes through unchanged.
    #[allow(clippy::result_large_err)]
    pub(crate) fn verify_auth(
        mut req: axum::extract::Request,
        verifier: Option<&dyn TokenVerifier>,
    ) -> Result<axum::extract::Request, axum::response::Response> {
        let Some(verifier) = verifier else {
            return Ok(req);
        };

        let token = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .ok_or_else(|| {
                let mut r = axum::response::Response::new(axum::body::Body::from(
                    "missing or malformed Authorization header",
                ));
                *r.status_mut() = StatusCode::UNAUTHORIZED;
                r.headers_mut().insert(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static("text/plain; charset=utf-8"),
                );
                r
            })?;

        let claims = verifier.verify(token).map_err(|e| {
            tracing::debug!(error = %e, "bearer token rejected");
            let mut r = axum::response::Response::new(axum::body::Body::from("invalid token"));
            *r.status_mut() = StatusCode::UNAUTHORIZED;
            r.headers_mut().insert(
                header::CONTENT_TYPE,
                HeaderValue::from_static("text/plain; charset=utf-8"),
            );
            r
        })?;

        let sub = claims.sub.clone().unwrap_or_default();
        let mut ctx =
            SecurityContext::authenticated_with(Box::new(JwtPrincipal { sub: sub.clone() }));
        if !sub.is_empty() {
            ctx = ctx.with_claim("sub", sub);
        }
        if let Some(iss) = &claims.iss {
            ctx = ctx.with_claim("iss", iss.clone());
        }
        if let Some(tenant) = claims.custom.get("tenant_id") {
            ctx = ctx.with_tenant(tenant.to_string().trim_matches('"').to_string());
        }
        for (k, v) in &claims.custom {
            ctx = ctx.with_claim(k.clone(), v.to_string());
        }
        req.extensions_mut().insert(Arc::new(ctx));
        Ok(req)
    }
}

#[cfg(test)]
mod tests {
    use super::{AxumHttpServerHelper, JwtPrincipal};
    use edge_domain::Principal;
    use swe_edge_ingress_verifier::{Claims, TokenVerifier, VerifierError};

    #[test]
    fn test_jwt_principal_id_returns_sub() {
        let p = JwtPrincipal {
            sub: "user-42".to_string(),
        };
        assert_eq!(p.id(), "user-42");
    }

    #[test]
    fn test_jwt_principal_kind_returns_jwt() {
        let p = JwtPrincipal { sub: String::new() };
        assert_eq!(p.kind(), "jwt");
    }

    struct JwtAcceptAll;
    impl TokenVerifier for JwtAcceptAll {
        fn verify(&self, _: &str) -> Result<Claims, VerifierError> {
            Ok(Claims::builder().sub("test-user").build())
        }
    }

    struct JwtDenyAll;
    impl TokenVerifier for JwtDenyAll {
        fn verify(&self, _: &str) -> Result<Claims, VerifierError> {
            Err(VerifierError::Invalid("denied".into()))
        }
    }

    #[test]
    fn test_verify_auth_no_verifier_passes_through_happy() {
        let req = axum::http::Request::builder()
            .uri("/")
            .body(axum::body::Body::empty())
            .unwrap();
        let result = AxumHttpServerHelper::verify_auth(req, None);
        assert!(result.is_ok(), "no verifier must pass request through");
    }

    #[test]
    fn test_verify_auth_with_verifier_missing_auth_header_returns_401_error() {
        let req = axum::http::Request::builder()
            .uri("/secure")
            .body(axum::body::Body::empty())
            .unwrap();
        let result = AxumHttpServerHelper::verify_auth(req, Some(&JwtAcceptAll));
        assert!(
            result.is_err(),
            "missing auth header with verifier must fail"
        );
        if let Err(resp) = result {
            assert_eq!(resp.status(), axum::http::StatusCode::UNAUTHORIZED);
        }
    }

    #[test]
    fn test_verify_auth_with_bearer_token_verified_ok_edge() {
        let req = axum::http::Request::builder()
            .uri("/secure")
            .header(axum::http::header::AUTHORIZATION, "Bearer validtoken")
            .body(axum::body::Body::empty())
            .unwrap();
        let result = AxumHttpServerHelper::verify_auth(req, Some(&JwtAcceptAll));
        assert!(
            result.is_ok(),
            "valid bearer token with JwtAcceptAll must pass"
        );
    }

    #[test]
    fn test_verify_auth_reject_invalid_token_deny_all() {
        let req = axum::http::Request::builder()
            .uri("/secure")
            .header(axum::http::header::AUTHORIZATION, "Bearer badtoken")
            .body(axum::body::Body::empty())
            .unwrap();
        let result = AxumHttpServerHelper::verify_auth(req, Some(&JwtDenyAll));
        assert!(result.is_err(), "JwtDenyAll must reject any token");
        if let Err(resp) = result {
            assert_eq!(resp.status(), axum::http::StatusCode::UNAUTHORIZED);
        }
    }
}
