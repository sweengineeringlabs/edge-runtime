//! Integration tests for SweEdgeRuntimeIsolatorFactory.

use swe_edge_runtime_isolator::SweEdgeRuntimeIsolatorFactory;

/// @covers: SweEdgeRuntimeIsolatorFactory
#[test]
fn test_swe_edge_runtime_isolator_factory_size_is_known() {
    let _ = std::mem::size_of::<SweEdgeRuntimeIsolatorFactory>();
}
