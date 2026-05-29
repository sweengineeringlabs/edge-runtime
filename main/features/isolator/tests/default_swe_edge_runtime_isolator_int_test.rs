//! DefaultSweEdgeRuntimeIsolator integration tests for swe-edge-runtime-isolator.

use swe_edge_runtime_isolator::{SweEdgeRuntimeIsolatorFactory, SweEdgeRuntimeIsolator};

/// @covers: SweEdgeRuntimeIsolatorFactory::create_swe_edge_runtime_isolator
#[test]
fn test_default_swe_edge_runtime_isolator_creates_and_executes() {
    let svc = SweEdgeRuntimeIsolatorFactory::create_swe_edge_runtime_isolator();
    assert!(svc.execute().is_ok());
}
