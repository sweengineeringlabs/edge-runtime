//! Integration tests for the `NoopGrpcIngress` type (api/types).

use swe_edge_runtime_grpc::{GrpcIngress, NoopGrpcIngress};

/// @covers: NoopGrpcIngress — is a zero-sized type
#[test]
fn test_noop_grpc_ingress_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<NoopGrpcIngress>(), 0);
}

/// @covers: NoopGrpcIngress — cannot construct an error path (documents absence)
#[test]
fn test_noop_grpc_ingress_construct_no_error_path_error() {
    // NoopGrpcIngress::create() is infallible; documents absence of an error path.
    let _a = NoopGrpcIngress::create();
}

/// @covers: NoopGrpcIngress — implements GrpcIngress trait
#[test]
fn test_noop_grpc_ingress_implements_grpc_ingress_edge() {
    fn _assert_impl<T: GrpcIngress>() {}
    _assert_impl::<NoopGrpcIngress>();
}
