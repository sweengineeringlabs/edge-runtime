//! Integration tests for the lifecycle_observer_svc SAF surface.

use swe_edge_runtime::LIFECYCLE_OBSERVER_SVC;

/// @covers: LIFECYCLE_OBSERVER_SVC
#[test]
fn test_lifecycle_observer_svc_slug_is_correct_happy() {
    assert_eq!(LIFECYCLE_OBSERVER_SVC, "lifecycle_observer");
}

#[test]
fn test_lifecycle_observer_svc_slug_is_non_empty_error() {
    assert!(!LIFECYCLE_OBSERVER_SVC.is_empty());
}

#[test]
fn test_lifecycle_observer_svc_slug_has_no_whitespace_edge() {
    assert!(!LIFECYCLE_OBSERVER_SVC.contains(char::is_whitespace));
}
