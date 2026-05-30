//! Integration tests for swe-edge-runtime-isolator.

use swe_edge_runtime_isolator::SweEdgeRuntimeIsolator;
use swe_edge_runtime_isolator::SweEdgeRuntimeIsolatorFactory;

/// @covers: SweEdgeRuntimeIsolatorFactory::create_swe_edge_runtime_isolator
#[test]
fn test_create_swe_edge_runtime_isolator_succeeds() {
    let svc = SweEdgeRuntimeIsolatorFactory::create_swe_edge_runtime_isolator();
    assert!(svc.execute().is_ok());
}
