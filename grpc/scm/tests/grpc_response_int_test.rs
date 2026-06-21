//! Integration tests for [`swe_edge_runtime_grpc::GrpcResponse`].

use swe_edge_runtime_grpc::GrpcResponse;

/// @covers: GrpcResponse::ok — happy path
#[test]
fn test_ok_sets_status_200_and_body_happy() {
    let resp = GrpcResponse::ok(b"data".to_vec());
    assert_eq!(resp.status, 0, "gRPC status 0 means OK");
    assert_eq!(resp.body, b"data");
}

/// @covers: GrpcResponse::empty — body is empty
#[test]
fn test_empty_body_is_empty_edge() {
    let resp = GrpcResponse::empty();
    assert!(resp.body.is_empty());
}

/// @covers: GrpcResponse::is_ok — true for status 0
#[test]
fn test_is_ok_true_for_status_zero_happy() {
    let resp = GrpcResponse::ok(vec![]);
    assert!(resp.is_ok());
}

/// @covers: GrpcResponse::is_ok — false for non-zero status
#[test]
fn test_is_ok_false_for_nonzero_status_error() {
    let resp = GrpcResponse {
        status: 13,
        body: vec![],
        metadata: Default::default(),
    };
    assert!(!resp.is_ok());
}

/// @covers: GrpcResponse::empty — metadata defaults to empty
#[test]
fn test_empty_metadata_empty_by_default_edge() {
    let resp = GrpcResponse::empty();
    assert!(resp.metadata.is_empty());
}
