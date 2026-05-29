//! Integration tests for the SweEdgeRuntimeIsolator service trait.

use swe_edge_runtime_isolator::SweEdgeRuntimeIsolator;

/// @covers: SweEdgeRuntimeIsolator
#[test]
fn test_swe_edge_runtime_isolator_trait_is_object_safe() {
    fn _accept(_s: &dyn SweEdgeRuntimeIsolator) {}
}
