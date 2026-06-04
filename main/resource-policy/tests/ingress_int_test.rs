//! Gateway ingress integration tests.

use swe_edge_runtime_resource_policy::Error;

/// @covers: ingress
#[test]
fn test_error_accessible_through_crate_root() {
    let _ = std::mem::size_of::<Error>();
}
