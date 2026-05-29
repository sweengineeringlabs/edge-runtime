//! Integration tests for IsolatorError.

use swe_edge_runtime_isolator::Error;

/// @covers: IsolatorError
#[test]
fn test_isolator_error_io_displays_correctly() {
    let err = Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "not found"));
    assert!(err.to_string().contains("I/O error"));
}

/// @covers: IsolatorError
#[test]
fn test_isolator_error_operation_displays_correctly() {
    let err = Error::Operation { message: "test failure".into() };
    assert!(err.to_string().contains("test failure"));
}
