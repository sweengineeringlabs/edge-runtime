//! Integration tests for [`swe_edge_runtime_grpc::GrpcRequest`].

use swe_edge_runtime_grpc::GrpcRequest;

/// @covers: GrpcRequest::new — happy path
#[test]
fn test_new_sets_service_method_body_happy() {
    let req = GrpcRequest::new("acme.Greeter", "SayHello", b"body".to_vec());
    assert_eq!(req.service, "acme.Greeter");
    assert_eq!(req.method, "SayHello");
    assert_eq!(req.body, b"body");
}

/// @covers: GrpcRequest::new — metadata defaults to empty
#[test]
fn test_new_metadata_empty_by_default_edge() {
    let req = GrpcRequest::new("svc", "Rpc", vec![]);
    assert!(req.metadata.is_empty());
}

/// @covers: GrpcRequest::with_metadata — attaches a metadata entry
#[test]
fn test_with_metadata_inserts_key_value_happy() {
    let req = GrpcRequest::new("svc", "Rpc", vec![]).with_metadata("x-token", "abc");
    assert_eq!(req.metadata.get("x-token").map(|s| s.as_str()), Some("abc"));
}

/// @covers: GrpcRequest::with_metadata — empty key is accepted
#[test]
fn test_with_metadata_empty_key_accepted_error() {
    let req = GrpcRequest::new("svc", "Rpc", vec![]).with_metadata("", "val");
    assert!(req.metadata.contains_key(""));
}

/// @covers: GrpcRequest construction with empty body
#[test]
fn test_new_empty_body_accepted_edge() {
    let req = GrpcRequest::new("svc", "Rpc", vec![]);
    assert!(req.body.is_empty());
}
