//! Tests for `HttpHealthCheck` — health check result type.
// @covers HttpHealthCheck
use swe_edge_runtime_http::HttpHealthCheck;

#[test]
fn test_http_health_check_healthy_happy() {
    let check = HttpHealthCheck::healthy();
    assert!(check.healthy);
    assert!(check.message.is_none());
}

#[test]
fn test_http_health_check_unhealthy_error() {
    let check = HttpHealthCheck::unhealthy("disk full");
    assert!(!check.healthy);
    assert_eq!(check.message.as_deref(), Some("disk full"));
}

#[test]
fn test_http_health_check_unhealthy_empty_message_edge() {
    // Edge: empty message string is a valid (if unusual) unhealthy state.
    let check = HttpHealthCheck::unhealthy("");
    assert!(!check.healthy);
    assert_eq!(check.message.as_deref(), Some(""));
}
