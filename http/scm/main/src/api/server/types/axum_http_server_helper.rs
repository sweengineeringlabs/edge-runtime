//! `AxumHttpServerHelper` — API-layer type for the Axum server.

use std::collections::HashMap;

use axum::http::{header, HeaderValue, StatusCode};

/// Helper struct for Axum HTTP server operations.
pub struct AxumHttpServerHelper;

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
