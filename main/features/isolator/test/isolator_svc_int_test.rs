//! Integration tests for swe-edge-runtime-isolator SAF facade.

use swe_edge_runtime_isolator::*;

/// @covers: create_swe_edge_runtime_isolator
#[test]
fn test_create_swe_edge_runtime_isolator_via_saf_succeeds() {
    let svc = create_swe_edge_runtime_isolator();
    assert!(svc.execute().is_ok());
}

/// @covers: create_validator
#[test]
fn test_create_validator_via_saf_succeeds() {
    let v = create_validator();
    assert!(v.validate().is_ok());
}
