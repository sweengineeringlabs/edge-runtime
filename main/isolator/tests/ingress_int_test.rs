//! Gateway ingress integration tests.

use swe_edge_runtime_isolator::Error;

/// @covers: ingress
#[test]
fn test_error_accessible_through_crate_root() {
    let _ = std::mem::size_of::<Error>();
}
