//! Integration tests for the SAF surface of `swe-edge-runtime-grpc`.

use swe_edge_runtime_grpc::{
    GrpcHealthCheck, GrpcIngress, GrpcIngressError, GrpcIngressResult, GrpcMethod, GrpcRequest,
    GrpcResponse, NoopGrpcIngress,
};

/// @covers: all SAF types are importable from crate root
#[test]
fn test_saf_types_importable_from_crate_root_happy() {
    fn _check(
        _: GrpcRequest,
        _: GrpcResponse,
        _: GrpcHealthCheck,
        _: GrpcMethod,
        _: GrpcIngressError,
        _: GrpcIngressResult<()>,
    ) {
    }
}

/// @covers: NoopGrpcIngress::create — returns a usable instance
#[test]
fn test_create_returns_noop_instance_happy() {
    let _noop: NoopGrpcIngress = NoopGrpcIngress::create();
}

/// @covers: NoopGrpcIngress implements GrpcIngress
#[test]
fn test_noop_implements_trait_happy() {
    fn _assert_impl<T: GrpcIngress>() {}
    _assert_impl::<NoopGrpcIngress>();
}

/// @covers: GrpcIngress is object-safe — can be erased to dyn
#[test]
fn test_grpc_ingress_trait_is_object_safe_edge() {
    fn _assert(_: &dyn GrpcIngress) {}
}

/// @covers: NoopGrpcIngress::create — no error path (documents absence)
#[test]
fn test_create_noop_is_zero_cost_error() {
    // NoopGrpcIngress::create() is infallible; this test documents the absence of an error path.
    let _a = NoopGrpcIngress::create();
    let _b = NoopGrpcIngress::create();
}

/// @covers: NoopGrpcIngress::create — two instances are independent
#[test]
fn test_create_two_instances_are_independent_edge() {
    let a = NoopGrpcIngress::create();
    let b = NoopGrpcIngress::create();
    fn _assert_impl<T: GrpcIngress>(_: &T) {}
    _assert_impl(&a);
    _assert_impl(&b);
}
