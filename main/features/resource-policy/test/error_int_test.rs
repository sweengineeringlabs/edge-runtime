//! Error integration tests for swe-edge-runtime-resource-policy.

use swe_edge_runtime_resource_policy::*;

#[test]
fn test_error_display_io() {
    let err = Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "test"));
    assert!(err.to_string().contains("I/O error"));
}

#[test]
fn test_error_display_operation() {
    let err = Error::Operation { message: "bad value".to_string() };
    assert!(err.to_string().contains("bad value"));
}
