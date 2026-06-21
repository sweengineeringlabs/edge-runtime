//! Integration tests for the http_load_monitor_svc SAF surface.

use swe_edge_runtime::{HttpLoadMonitor, HTTP_LOAD_MONITOR_SVC};

/// @covers: HTTP_LOAD_MONITOR_SVC
#[test]
fn test_http_load_monitor_svc_slug_is_correct_happy() {
    assert_eq!(HTTP_LOAD_MONITOR_SVC, "http_load_monitor");
}

#[test]
fn test_http_load_monitor_svc_slug_is_non_empty_error() {
    assert!(!HTTP_LOAD_MONITOR_SVC.is_empty());
}

#[test]
fn test_http_load_monitor_is_object_safe_edge() {
    fn _accept(_: &dyn HttpLoadMonitor) {}
}
