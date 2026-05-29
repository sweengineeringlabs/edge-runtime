//! Integration tests for ConfigValidator.

use swe_edge_runtime::{load_config, validate_config};

/// @covers: config_validator
#[test]
fn test_config_validator_accepts_default_runtime_config() {
    let cfg = load_config().expect("load_config");
    assert!(validate_config(&cfg).is_ok());
}

/// @covers: config_validator
#[test]
fn test_config_validator_accepts_explicit_valid_config() {
    use swe_edge_runtime::RuntimeConfig;
    let cfg = RuntimeConfig::default().with_http_bind("0.0.0.0:8080");
    assert!(validate_config(&cfg).is_ok());
}
