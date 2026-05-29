//! Unit tests for swe-edge-runtime-isolator.

use swe_edge_runtime_isolator::*;

/// @covers: create_swe_edge_runtime_isolator
#[test]
fn test_create_swe_edge_runtime_isolator_returns_working_impl() {
    let svc = create_swe_edge_runtime_isolator();
    assert!(svc.execute().is_ok());
}
