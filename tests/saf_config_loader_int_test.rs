//! Public-API integration tests for saf config-loading functions.

use swe_edge_runtime::{
    load_config, load_config_from, load_config_xdg, load_tenant_config, load_tenant_config_from,
    load_tenant_config_xdg, validate_config,
};

/// @covers: load_config
#[test]
fn test_load_config_returns_valid_runtime_config() {
    let cfg = load_config().expect("load_config");
    assert!(!cfg.http_bind.is_empty());
}

/// @covers: load_config_from
#[test]
fn test_load_config_from_temp_dir_returns_defaults() {
    let dir = tempfile::tempdir().unwrap();
    let cfg = load_config_from(dir.path()).expect("load_config_from");
    assert!(!cfg.http_bind.is_empty());
}

/// @covers: load_config_xdg
#[test]
fn test_load_config_xdg_nonexistent_app_returns_defaults() {
    let cfg = load_config_xdg("swe-edge-test-xyz-nonexistent").expect("load_config_xdg");
    assert!(!cfg.grpc_bind.is_empty());
}

/// @covers: load_tenant_config
#[test]
fn test_load_tenant_config_unknown_tenant_returns_error() {
    let result = load_tenant_config("nonexistent-tenant-xyz");
    assert!(result.is_err(), "expected error for unknown tenant");
}

/// @covers: load_tenant_config_from
#[test]
fn test_load_tenant_config_from_missing_file_returns_error() {
    let dir = tempfile::tempdir().unwrap();
    let result = load_tenant_config_from("no-tenant", dir.path());
    assert!(result.is_err());
}

/// @covers: load_tenant_config_xdg
#[test]
fn test_load_tenant_config_xdg_missing_returns_error() {
    let result = load_tenant_config_xdg("swe-edge-test-xyz-nonexistent", "no-tenant");
    assert!(result.is_err());
}

/// @covers: validate_config
#[test]
fn test_validate_config_accepts_valid_default_config() {
    let cfg = load_config().unwrap();
    assert!(validate_config(&cfg).is_ok());
}
