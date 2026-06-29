//! Public header-inspection and response-building methods for [`crate::api::AxumHttpServerHelper`].

use std::collections::HashMap;

use axum::http::{header, HeaderValue, StatusCode};

use crate::api::AxumHttpServerHelper;

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
        Self::text_response(
            StatusCode::PAYLOAD_TOO_LARGE,
            "request body exceeds size limit",
        )
    }

    /// Build a `500 Internal Server Error` response.
    pub fn internal_server_error(msg: &'static str) -> axum::response::Response {
        Self::text_response(StatusCode::INTERNAL_SERVER_ERROR, msg)
    }

    fn text_response(status: StatusCode, body: &'static str) -> axum::response::Response {
        let mut response = axum::response::Response::new(axum::body::Body::from(body));
        *response.status_mut() = status;
        response.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("text/plain; charset=utf-8"),
        );
        response
    }
}

#[cfg(test)]
mod tests {
    use axum::http::{header, HeaderMap, HeaderValue, StatusCode};

    use super::*;

    /// @covers: text_response
    #[test]
    fn test_text_response_sets_status_and_content_type() {
        let r = AxumHttpServerHelper::text_response(StatusCode::IM_A_TEAPOT, "brew");
        assert_eq!(r.status(), StatusCode::IM_A_TEAPOT);
        let ct = r
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        assert!(ct.contains("text/plain"), "must be text/plain; got: {ct}");
    }

    /// @covers: text_response
    #[test]
    fn test_text_response_different_statuses_are_independent() {
        let r200 = AxumHttpServerHelper::text_response(StatusCode::OK, "ok");
        let r404 = AxumHttpServerHelper::text_response(StatusCode::NOT_FOUND, "nf");
        assert_ne!(r200.status(), r404.status(), "status codes must differ");
    }

    /// @covers: is_websocket_upgrade
    #[test]
    fn test_is_websocket_upgrade_detects_header() {
        let mut h = HeaderMap::new();
        h.insert(header::UPGRADE, HeaderValue::from_static("websocket"));
        assert!(AxumHttpServerHelper::is_websocket_upgrade(&h));
        assert!(!AxumHttpServerHelper::is_websocket_upgrade(
            &HeaderMap::new()
        ));
    }

    /// @covers: is_sse_request
    #[test]
    fn test_is_sse_request_detects_accept() {
        let mut h = HeaderMap::new();
        h.insert(
            header::ACCEPT,
            HeaderValue::from_static("text/event-stream"),
        );
        assert!(AxumHttpServerHelper::is_sse_request(&h));
        assert!(!AxumHttpServerHelper::is_sse_request(&HeaderMap::new()));
    }

    /// @covers: collect_headers
    #[test]
    fn test_collect_headers_returns_entries() {
        let mut h = HeaderMap::new();
        h.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        let map = AxumHttpServerHelper::collect_headers(&h);
        assert_eq!(
            map.get("content-type").map(String::as_str),
            Some("application/json")
        );
    }

    /// @covers: payload_too_large
    #[test]
    fn test_payload_too_large_returns_413() {
        let r = AxumHttpServerHelper::payload_too_large();
        assert_eq!(r.status(), StatusCode::PAYLOAD_TOO_LARGE);
    }

    /// @covers: internal_server_error
    #[test]
    fn test_internal_server_error_returns_500() {
        let r = AxumHttpServerHelper::internal_server_error("oops");
        assert_eq!(r.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}
