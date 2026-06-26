//! SAF factory surface for [`HttpServer`] — construction and configuration.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use axum::http::{header, HeaderValue, StatusCode};
use swe_edge_ingress_http::{HttpIngress, HttpStream, DEFAULT_REQUEST_TIMEOUT, MAX_BODY_BYTES};
use swe_edge_ingress_tls::IngressTlsConfig;
use swe_edge_ingress_verifier::TokenVerifier;

use crate::api::{AxumHttpServer, AxumHttpServerBuilder, AxumHttpServerHelper};

/// Service identifier for the HTTP server factory.
pub const HTTP_SERVER_SVC_FACTORY: &str = "http_server";

// ── AxumHttpServer constructors ───────────────────────────────────────────────

impl AxumHttpServer {
    /// Create a server binding to `bind` and delegating requests to `handler`.
    pub fn new(bind: impl Into<String>, handler: Arc<dyn HttpIngress>) -> Self {
        Self {
            bind: bind.into(),
            handler,
            body_limit: MAX_BODY_BYTES,
            request_timeout: DEFAULT_REQUEST_TIMEOUT,
            tls: None,
            bearer_verifier: None,
            stream_handler: None,
        }
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
    pub fn with_tls(mut self, config: IngressTlsConfig) -> Self {
        self.tls = Some(config);
        self
    }

    /// Enable JWT bearer authentication.
    pub fn with_bearer_auth(mut self, verifier: Arc<dyn TokenVerifier>) -> Self {
        self.bearer_verifier = Some(verifier);
        self
    }
}

// ── AxumHttpServerBuilder ─────────────────────────────────────────────────────

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
    pub fn with_tls(mut self, config: IngressTlsConfig) -> Self {
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
        let mut s = AxumHttpServer::new(self.bind, self.handler)
            .with_body_limit(self.body_limit)
            .with_request_timeout(self.request_timeout);
        if let Some(tls) = self.tls {
            s = s.with_tls(tls);
        }
        if let Some(v) = self.bearer_verifier {
            s = s.with_bearer_auth(v);
        }
        s
    }
}

// ── AxumHttpServerHelper public surface ──────────────────────────────────────

impl AxumHttpServerHelper {
    /// Returns `true` if the request carries a WebSocket upgrade header.
    pub fn is_websocket_upgrade(headers: &axum::http::HeaderMap) -> bool {
        headers
            .get(axum::http::header::UPGRADE)
            .and_then(|v| v.to_str().ok())
            .map(|v| v.eq_ignore_ascii_case("websocket"))
            .unwrap_or(false)
    }

    /// Returns `true` if the request accepts `text/event-stream` (SSE).
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
        let mut response = axum::response::Response::new(axum::body::Body::from(
            "request body exceeds size limit",
        ));
        *response.status_mut() = StatusCode::PAYLOAD_TOO_LARGE;
        response.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("text/plain; charset=utf-8"),
        );
        response
    }

    /// Build a `500 Internal Server Error` response.
    pub fn internal_server_error(msg: &'static str) -> axum::response::Response {
        let mut response = axum::response::Response::new(axum::body::Body::from(msg));
        *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
        response.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("text/plain; charset=utf-8"),
        );
        response
    }
}
