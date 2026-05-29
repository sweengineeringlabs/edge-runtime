//! Gateway egress integration tests.

use swe_edge_runtime_isolator::SweEdgeRuntimeIsolatorFactory;

/// @covers: egress
#[test]
fn test_factory_accessible_through_crate_root() {
    let _ = std::mem::size_of::<SweEdgeRuntimeIsolatorFactory>();
}
