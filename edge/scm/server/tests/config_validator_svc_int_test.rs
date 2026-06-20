//! Integration tests for the config_validator_svc SAF surface.

use swe_edge_runtime::CONFIG_VALIDATOR_SVC;

/// @covers: CONFIG_VALIDATOR_SVC
#[test]
fn test_config_validator_svc_slug_is_correct_happy() {
    assert_eq!(CONFIG_VALIDATOR_SVC, "config_validator");
}

#[test]
fn test_config_validator_svc_slug_is_non_empty_error() {
    assert!(!CONFIG_VALIDATOR_SVC.is_empty());
}

#[test]
fn test_config_validator_svc_slug_has_no_whitespace_edge() {
    assert!(!CONFIG_VALIDATOR_SVC.contains(char::is_whitespace));
}
