//! Integration tests for DefaultSweEdgeRuntimeIsolatorImpl.

use swe_edge_runtime_isolator::{SweEdgeRuntimeIsolator, SweEdgeRuntimeIsolatorFactory};

/// @covers: DefaultSweEdgeRuntimeIsolatorImpl
#[test]
fn test_default_isolator_impl_is_accessible_via_factory() {
    // Verifies the type exists and factory creates a usable instance.
    let svc = SweEdgeRuntimeIsolatorFactory::create_swe_edge_runtime_isolator();
    assert!(svc.execute().is_ok());
}

/// @covers: DefaultSweEdgeRuntimeIsolatorImpl
#[test]
fn test_default_isolator_impl_type_size_is_known() {
    use swe_edge_runtime_isolator::DefaultSweEdgeRuntimeIsolatorImpl;
    let _ = std::mem::size_of::<DefaultSweEdgeRuntimeIsolatorImpl>();
}
