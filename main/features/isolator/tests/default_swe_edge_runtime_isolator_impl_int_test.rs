//! Integration tests for the default SweEdgeRuntimeIsolator implementation.

use swe_edge_runtime_isolator::{SweEdgeRuntimeIsolator, SweEdgeRuntimeIsolatorFactory};

/// @covers: SweEdgeRuntimeIsolatorFactory::create_swe_edge_runtime_isolator
#[test]
fn test_default_isolator_impl_is_accessible_via_factory() {
    // Verifies the factory produces a usable SweEdgeRuntimeIsolator implementation.
    let svc = SweEdgeRuntimeIsolatorFactory::create_swe_edge_runtime_isolator();
    assert!(svc.execute().is_ok());
}

/// @covers: SweEdgeRuntimeIsolatorFactory::create_swe_edge_runtime_isolator
#[test]
fn test_default_isolator_impl_execute_is_idempotent() {
    let svc = SweEdgeRuntimeIsolatorFactory::create_swe_edge_runtime_isolator();
    assert!(svc.execute().is_ok(), "first execute must succeed");
    assert!(
        svc.execute().is_ok(),
        "second execute must succeed (idempotent)"
    );
}
