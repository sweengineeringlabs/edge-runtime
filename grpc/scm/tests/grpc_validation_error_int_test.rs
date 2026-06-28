//! Integration tests for [`GrpcValidationError`].
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_runtime_grpc::GrpcValidationError;

/// @covers: grpc_validation_error
#[test]
fn test_grpc_validation_error_invalid_constructs_happy() {
    let err = GrpcValidationError::Invalid("bad input".to_string());
    assert!(
        matches!(err, GrpcValidationError::Invalid(_)),
        "Invalid variant must be constructible"
    );
}

/// @covers: grpc_validation_error
#[test]
fn test_grpc_validation_error_debug_contains_message_error() {
    let err = GrpcValidationError::Invalid("must not be empty".to_string());
    let debug = format!("{err:?}");
    assert!(
        debug.contains("must not be empty"),
        "Debug output must contain the error message"
    );
}

/// @covers: grpc_validation_error
#[test]
fn test_grpc_validation_error_equality_distinguishes_messages_edge() {
    let a = GrpcValidationError::Invalid("foo".to_string());
    let b = GrpcValidationError::Invalid("bar".to_string());
    let same = GrpcValidationError::Invalid("foo".to_string());
    assert_ne!(a, b, "errors with different messages must not be equal");
    assert_eq!(a, same, "errors with identical messages must be equal");
}
