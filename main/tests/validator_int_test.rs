//! Integration tests for the public `validate_config` function.

use swe_edge_runtime::{validate_config, RuntimeConfig};

/// @covers: validate_config — valid default config passes
#[test]
fn test_validate_config_accepts_default_config() {
    let config = RuntimeConfig::default();
    assert!(validate_config(&config).is_ok());
}

/// @covers: validate_config — empty http_bind is rejected
#[test]
fn test_validate_config_rejects_empty_http_bind() {
    let mut config = RuntimeConfig::default();
    config.http_bind = "".into();
    assert!(validate_config(&config).is_err());
}

/// @covers: validate_config — zero shutdown timeout is rejected
#[test]
fn test_validate_config_rejects_zero_shutdown_timeout() {
    let mut config = RuntimeConfig::default();
    config.shutdown_timeout_secs = 0;
    assert!(validate_config(&config).is_err());
}
