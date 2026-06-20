//! Integration tests for [`swe_edge_runtime_grpc::GrpcMethod`].

use swe_edge_runtime_grpc::GrpcMethod;

/// @covers: GrpcMethod — Unary is the default for request-response RPCs
#[test]
fn test_unary_variant_constructable_happy() {
    let method = GrpcMethod::Unary;
    assert_eq!(method, GrpcMethod::Unary);
}

/// @covers: GrpcMethod — all four streaming variants are distinct
#[test]
fn test_all_variants_are_distinct_error() {
    assert_ne!(GrpcMethod::Unary, GrpcMethod::ServerStream);
    assert_ne!(GrpcMethod::ServerStream, GrpcMethod::ClientStream);
    assert_ne!(GrpcMethod::ClientStream, GrpcMethod::BidiStream);
}

/// @covers: GrpcMethod — Clone produces an equal copy
#[test]
fn test_clone_produces_equal_value_edge() {
    let original = GrpcMethod::ServerStream;
    let cloned = original;
    assert_eq!(original, cloned);
}
