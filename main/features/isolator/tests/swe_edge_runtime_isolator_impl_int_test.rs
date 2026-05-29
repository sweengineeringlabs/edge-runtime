//! Integration tests for DefaultSweEdgeRuntimeIsolatorImpl.

use swe_edge_runtime_isolator::{SweEdgeRuntimeIsolatorFactory, SweEdgeRuntimeIsolator};

/// @covers: DefaultSweEdgeRuntimeIsolatorImpl
#[test]
fn test_default_impl_executes() {
    let svc = SweEdgeRuntimeIsolatorFactory::create_swe_edge_runtime_isolator();
    assert!(svc.execute().is_ok());
}
