//! Integration tests for the application_config_loader_svc SAF surface.

use swe_edge_runtime::APPLICATION_CONFIG_LOADER_SVC;

/// @covers: APPLICATION_CONFIG_LOADER_SVC
#[test]
fn test_application_config_loader_svc_slug_is_correct_happy() {
    assert_eq!(APPLICATION_CONFIG_LOADER_SVC, "application_config_loader");
}

#[test]
fn test_application_config_loader_svc_slug_is_non_empty_error() {
    assert!(!APPLICATION_CONFIG_LOADER_SVC.is_empty());
}

#[test]
fn test_application_config_loader_svc_slug_has_no_whitespace_edge() {
    assert!(!APPLICATION_CONFIG_LOADER_SVC.contains(char::is_whitespace));
}
