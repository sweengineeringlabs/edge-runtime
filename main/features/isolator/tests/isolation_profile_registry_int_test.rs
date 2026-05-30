//! Integration tests for IsolationProfileRegistry.

use swe_edge_egress_subprocess::IsolationError;
use swe_edge_runtime_isolator::{IsolatorConfig, IsolatorSvc};

/// @covers: IsolationProfileRegistry::get
#[test]
fn test_isolation_profile_registry_get_noop_returns_ok() {
    let registry = IsolatorSvc::build_registry(IsolatorConfig::default()).expect("registry build");
    assert!(registry.get("noop").is_ok());
}

/// @covers: IsolationProfileRegistry::get
#[test]
fn test_isolation_profile_registry_get_unknown_returns_error() {
    let registry = IsolatorSvc::build_registry(IsolatorConfig::default()).expect("registry build");
    let err = registry.get("unknown_profile").unwrap_err();
    assert!(matches!(err, IsolationError::UnknownProfile { .. }));
}

/// @covers: IsolationProfileRegistry::len
#[test]
fn test_isolation_profile_registry_len_at_least_two() {
    let registry = IsolatorSvc::build_registry(IsolatorConfig::default()).expect("registry build");
    assert!(registry.len() >= 2);
}

/// @covers: IsolationProfileRegistry::is_empty
#[test]
fn test_isolation_profile_registry_is_not_empty() {
    let registry = IsolatorSvc::build_registry(IsolatorConfig::default()).expect("registry build");
    assert!(!registry.is_empty());
}
