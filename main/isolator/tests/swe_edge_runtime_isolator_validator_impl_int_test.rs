//! Integration tests for DefaultSweEdgeRuntimeIsolatorValidatorImpl.

use swe_edge_runtime_isolator::{SweEdgeRuntimeIsolatorFactory, Validator};

/// @covers: DefaultSweEdgeRuntimeIsolatorValidatorImpl
#[test]
fn test_default_validator_impl_validates() {
    let v = SweEdgeRuntimeIsolatorFactory::create_validator();
    assert!(v.validate().is_ok());
}
