//! Integration tests for swe-edge-runtime-isolator SAF facade.

use swe_edge_runtime_isolator::{SweEdgeRuntimeIsolatorFactory, SweEdgeRuntimeIsolator, Validator};

/// @covers: SweEdgeRuntimeIsolatorFactory::create_swe_edge_runtime_isolator
#[test]
fn test_create_swe_edge_runtime_isolator_via_saf_succeeds() {
    let svc = SweEdgeRuntimeIsolatorFactory::create_swe_edge_runtime_isolator();
    assert!(svc.execute().is_ok());
}

/// @covers: SweEdgeRuntimeIsolatorFactory::create_validator
#[test]
fn test_create_validator_via_saf_succeeds() {
    let v = SweEdgeRuntimeIsolatorFactory::create_validator();
    assert!(v.validate().is_ok());
}
