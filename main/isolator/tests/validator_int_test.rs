//! Integration tests for the Validator trait.

use swe_edge_runtime_isolator::{SweEdgeRuntimeIsolatorFactory, Validator};

/// @covers: Validator
#[test]
fn test_validator_validates_successfully() {
    let v = SweEdgeRuntimeIsolatorFactory::create_validator();
    assert!(v.validate().is_ok());
}
