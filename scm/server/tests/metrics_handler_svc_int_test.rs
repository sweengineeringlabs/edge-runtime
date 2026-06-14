//! Integration tests for the metrics_handler_svc SAF surface.

use swe_edge_runtime::METRICS_HANDLER_SVC;

/// @covers: METRICS_HANDLER_SVC
#[test]
fn test_metrics_handler_svc_slug_is_correct_happy() {
    assert_eq!(METRICS_HANDLER_SVC, "metrics_handler");
}

#[test]
fn test_metrics_handler_svc_slug_is_non_empty_error() {
    assert!(!METRICS_HANDLER_SVC.is_empty());
}

#[test]
fn test_metrics_handler_svc_slug_has_no_whitespace_edge() {
    assert!(!METRICS_HANDLER_SVC.contains(char::is_whitespace));
}
