//! Colocated tests for [`crate::api::AxumHttpServerHelper`] public methods.

#[cfg(test)]
mod tests {
    use axum::http::{header, HeaderMap, HeaderValue};

    use crate::api::AxumHttpServerHelper;

    /// @covers: is_websocket_upgrade
    #[test]
    fn test_is_websocket_upgrade_with_upgrade_header_returns_true_happy() {
        let mut headers = HeaderMap::new();
        headers.insert(header::UPGRADE, HeaderValue::from_static("websocket"));
        assert!(AxumHttpServerHelper::is_websocket_upgrade(&headers));
    }

    /// @covers: is_websocket_upgrade
    #[test]
    fn test_is_websocket_upgrade_without_header_returns_false_error() {
        assert!(!AxumHttpServerHelper::is_websocket_upgrade(
            &HeaderMap::new()
        ));
    }

    /// @covers: is_sse_request
    #[test]
    fn test_is_sse_request_with_event_stream_accept_returns_true_happy() {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::ACCEPT,
            HeaderValue::from_static("text/event-stream"),
        );
        assert!(AxumHttpServerHelper::is_sse_request(&headers));
    }

    /// @covers: is_sse_request
    #[test]
    fn test_is_sse_request_without_accept_returns_false_error() {
        assert!(!AxumHttpServerHelper::is_sse_request(&HeaderMap::new()));
    }

    /// @covers: collect_headers
    #[test]
    fn test_collect_headers_returns_key_value_pairs_happy() {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        let map = AxumHttpServerHelper::collect_headers(&headers);
        assert!(map.contains_key("content-type"));
    }

    /// @covers: collect_headers
    #[test]
    fn test_collect_headers_empty_map_returns_empty_edge() {
        let map = AxumHttpServerHelper::collect_headers(&HeaderMap::new());
        assert!(map.is_empty());
    }

    /// @covers: payload_too_large
    #[test]
    fn test_payload_too_large_returns_413_happy() {
        let resp = AxumHttpServerHelper::payload_too_large();
        assert_eq!(resp.status(), axum::http::StatusCode::PAYLOAD_TOO_LARGE);
    }

    /// @covers: payload_too_large
    #[test]
    fn test_payload_too_large_has_plain_text_content_type_error() {
        let resp = AxumHttpServerHelper::payload_too_large();
        let ct = resp
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        assert!(ct.contains("text/plain"), "must set text/plain; got: {ct}");
    }

    /// @covers: internal_server_error
    #[test]
    fn test_internal_server_error_returns_500_happy() {
        let resp = AxumHttpServerHelper::internal_server_error("oops");
        assert_eq!(resp.status(), axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    /// @covers: internal_server_error
    #[test]
    fn test_internal_server_error_has_plain_text_content_type_edge() {
        let resp = AxumHttpServerHelper::internal_server_error("err");
        let ct = resp
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        assert!(ct.contains("text/plain"), "must set text/plain; got: {ct}");
    }
}
