//! Integration tests for DefaultSweEdgeRuntimeIsolatorValidatorImpl.

use swe_edge_runtime_isolator::{SweEdgeRuntimeIsolatorFactory, Validator};

/// @covers: DefaultSweEdgeRuntimeIsolatorValidatorImpl
#[test]
fn test_default_validator_impl_is_accessible_via_factory() {
    let v = SweEdgeRuntimeIsolatorFactory::create_validator();
    assert!(v.validate().is_ok());
}

/// @covers: DefaultSweEdgeRuntimeIsolatorValidatorImpl
#[test]
fn test_default_validator_impl_type_size_is_known() {
    use swe_edge_runtime_isolator::DefaultSweEdgeRuntimeIsolatorValidatorImpl;
    let _ = std::mem::size_of::<DefaultSweEdgeRuntimeIsolatorValidatorImpl>();
}
