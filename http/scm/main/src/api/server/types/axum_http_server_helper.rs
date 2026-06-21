//! `AxumHttpServerHelper` — API-layer type and pure helpers for the Axum server.

use std::collections::HashMap;

use axum::http::StatusCode;

use swe_edge_ingress_http::{HttpBody, HttpIngressError, HttpMethod, HttpResponse};
use swe_edge_ingress_verifier::TokenVerifier;

use edge_domain::SecurityContext;

/// Helper struct for Axum HTTP server operations.
pub struct AxumHttpServerHelper;

struct JwtPrincipal {
    sub: String,
}

impl edge_domain::Principal for JwtPrincipal {
    fn id(&self) -> &str {
        &self.sub
    }
    fn kind(&self) -> &str {
        const KIND: &str = "jwt";
        KIND
    }
}

impl AxumHttpServerHelper {
    /// Check if request carries a `Upgrade: websocket` header.
    pub fn is_websocket_upgrade(headers: &axum::http::HeaderMap) -> bool {
        headers
            .get(axum::http::header::UPGRADE)
            .and_then(|v| v.to_str().ok())
            .map(|v| v.eq_ignore_ascii_case("websocket"))
            .unwrap_or(false)
    }

    /// Check if request accepts `text/event-stream` (SSE).
    pub fn is_sse_request(headers: &axum::http::HeaderMap) -> bool {
        headers
            .get(axum::http::header::ACCEPT)
            .and_then(|v| v.to_str().ok())
            .map(|v| v.contains("text/event-stream"))
            .unwrap_or(false)
    }

    /// Collect HTTP headers into a `HashMap<String, String>`.
    pub fn collect_headers(headers: &axum::http::HeaderMap) -> HashMap<String, String> {
        headers
            .iter()
            .filter_map(|(k, v)| v.to_str().ok().map(|vs| (k.to_string(), vs.to_string())))
            .collect()
    }

    /// Build a `413 Payload Too Large` response.
    pub fn payload_too_large() -> axum::response::Response {
        Self::plain_text_response(
            axum::http::StatusCode::PAYLOAD_TOO_LARGE,
            "request body exceeds size limit",
        )
    }

    /// Build a `500 Internal Server Error` response.
    pub fn internal_server_error(msg: &'static str) -> axum::response::Response {
        Self::plain_text_response(axum::http::StatusCode::INTERNAL_SERVER_ERROR, msg)
    }

    pub(crate) fn plain_text_response(
        status: axum::http::StatusCode,
        body: impl Into<String>,
    ) -> axum::response::Response {
        let mut response = axum::response::Response::new(axum::body::Body::from(body.into()));
        *response.status_mut() = status;
        response.headers_mut().insert(
            axum::http::header::CONTENT_TYPE,
            axum::http::HeaderValue::from_static("text/plain; charset=utf-8"),
        );
        response
    }

    /// Verify bearer token and attach a [`SecurityContext`] to the request.
    ///
    /// If no verifier is configured the request passes through unchanged.
    #[allow(clippy::result_large_err)]
    pub fn verify_auth(
        mut req: axum::extract::Request,
        verifier: Option<&dyn TokenVerifier>,
    ) -> Result<axum::extract::Request, axum::response::Response> {
        use std::sync::Arc;

        let Some(verifier) = verifier else {
            return Ok(req);
        };

        let token = req
            .headers()
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .ok_or_else(|| {
                Self::plain_text_response(
                    StatusCode::UNAUTHORIZED,
                    "missing or malformed Authorization header",
                )
            })?;

        let claims = verifier.verify(token).map_err(|e| {
            tracing::debug!(error = %e, "bearer token rejected");
            Self::plain_text_response(StatusCode::UNAUTHORIZED, "invalid token")
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

    /// Map HTTP method to [`HttpMethod`] enum.
    pub(crate) fn map_method(m: &axum::http::Method) -> HttpMethod {
        match *m {
            axum::http::Method::GET => HttpMethod::Get,
            axum::http::Method::POST => HttpMethod::Post,
            axum::http::Method::PUT => HttpMethod::Put,
            axum::http::Method::PATCH => HttpMethod::Patch,
            axum::http::Method::DELETE => HttpMethod::Delete,
            axum::http::Method::HEAD => HttpMethod::Head,
            axum::http::Method::OPTIONS => HttpMethod::Options,
            _ => HttpMethod::Get,
        }
    }

    /// Parse query string into `HashMap<String, String>`.
    pub(crate) fn parse_query(raw: Option<&str>) -> HashMap<String, String> {
        let mut map = HashMap::new();
        if let Some(q) = raw {
            for pair in q.split('&') {
                let mut parts = pair.splitn(2, '=');
                let key = Self::percent_decode(parts.next().unwrap_or(""));
                let value = Self::percent_decode(parts.next().unwrap_or(""));
                if !key.is_empty() {
                    map.insert(key, value);
                }
            }
        }
        map
    }

    /// Build [`HttpBody`] from raw bytes and content-type string.
    pub(crate) fn build_body(bytes: &bytes::Bytes, content_type: &str) -> Option<HttpBody> {
        if bytes.is_empty() {
            return None;
        }
        if content_type.contains("application/json") {
            serde_json::from_slice(bytes)
                .ok()
                .map(HttpBody::Json)
                .or_else(|| Some(HttpBody::Raw(bytes.to_vec())))
        } else if content_type.contains("application/x-www-form-urlencoded") {
            Some(HttpBody::Form(Self::parse_form(bytes)))
        } else {
            Some(HttpBody::Raw(bytes.to_vec()))
        }
    }

    /// Parse form data from bytes.
    pub(crate) fn parse_form(bytes: &bytes::Bytes) -> HashMap<String, String> {
        let mut map = HashMap::new();
        let s = std::str::from_utf8(bytes).unwrap_or("");
        for pair in s.split('&') {
            let mut parts = pair.splitn(2, '=');
            let key = Self::percent_decode(parts.next().unwrap_or(""));
            let value = Self::percent_decode(parts.next().unwrap_or(""));
            if !key.is_empty() {
                map.insert(key, value);
            }
        }
        map
    }

    /// Minimal percent-decode: `+` → space, `%XX` → byte.
    pub(crate) fn percent_decode(s: &str) -> String {
        let s = s.replace('+', " ");
        let mut out = String::with_capacity(s.len());
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '%' {
                let h1 = chars.next();
                let h2 = chars.next();
                match (h1, h2) {
                    (Some(a), Some(b)) => {
                        if let Ok(byte) = u8::from_str_radix(&format!("{a}{b}"), 16) {
                            out.push(byte as char);
                        } else {
                            out.push('%');
                            out.push(a);
                            out.push(b);
                        }
                    }
                    (Some(a), None) => {
                        out.push('%');
                        out.push(a);
                    }
                    _ => {
                        out.push('%');
                    }
                }
                continue;
            }
            out.push(c);
        }
        out
    }

    /// Build an Axum response from [`HttpResponse`].
    pub(crate) fn build_response(resp: HttpResponse) -> axum::response::Response {
        let status = axum::http::StatusCode::from_u16(resp.status)
            .unwrap_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        let mut builder = axum::response::Response::builder().status(status);
        for (k, v) in &resp.headers {
            builder = builder.header(k.as_str(), v.as_str());
        }
        builder
            .body(axum::body::Body::from(resp.body))
            .unwrap_or_else(|_| Self::internal_server_error("response build failed"))
    }

    /// Build an error response from [`HttpIngressError`].
    pub(crate) fn error_response(e: HttpIngressError) -> axum::response::Response {
        let status = match &e {
            HttpIngressError::NotFound(_) => axum::http::StatusCode::NOT_FOUND,
            HttpIngressError::InvalidInput(_) => axum::http::StatusCode::BAD_REQUEST,
            HttpIngressError::Unauthorized(_) => axum::http::StatusCode::UNAUTHORIZED,
            HttpIngressError::PermissionDenied(_) => axum::http::StatusCode::FORBIDDEN,
            HttpIngressError::Conflict(_) => axum::http::StatusCode::CONFLICT,
            HttpIngressError::MethodNotAllowed(_) => axum::http::StatusCode::METHOD_NOT_ALLOWED,
            HttpIngressError::UnprocessableEntity(_) => {
                axum::http::StatusCode::UNPROCESSABLE_ENTITY
            }
            HttpIngressError::Timeout(_) => axum::http::StatusCode::GATEWAY_TIMEOUT,
            HttpIngressError::Unavailable(_) => axum::http::StatusCode::SERVICE_UNAVAILABLE,
            HttpIngressError::Internal(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        axum::response::Response::builder()
            .status(status)
            .header("content-type", "text/plain; charset=utf-8")
            .body(axum::body::Body::from(e.to_string()))
            .unwrap_or_else(|_| Self::internal_server_error("error response build failed"))
    }

    /// Build a `408 Request Timeout` response.
    pub(crate) fn request_timeout_response() -> axum::response::Response {
        Self::plain_text_response(axum::http::StatusCode::REQUEST_TIMEOUT, "request timed out")
    }
}
