//! Integration tests for NoopGrpcIngress factory surface (saf/grpc/noop/grpc_ingress_svc_factory.rs).
#![allow(clippy::unwrap_used)]

use std::sync::Arc;

use swe_edge_runtime_grpc::NoopGrpcIngress;

// ── create ──────────────────────────────────────────────────────────────────

#[test]
fn test_create_returns_non_null_arc_happy() {
    // @covers: create
    let arc = NoopGrpcIngress::create();
    assert_eq!(
        Arc::strong_count(&arc),
        1,
        "fresh create() must have exactly one strong reference"
    );
}

#[test]
fn test_create_independent_instances_have_count_one_error() {
    // @covers: create
    let a = NoopGrpcIngress::create();
    let b = NoopGrpcIngress::create();
    assert_eq!(
        Arc::strong_count(&a),
        1,
        "first instance must have strong count 1"
    );
    assert_eq!(
        Arc::strong_count(&b),
        1,
        "second instance must have strong count 1"
    );
    assert_ne!(Arc::strong_count(&a), 0);
}

#[test]
fn test_create_successive_calls_produce_distinct_arcs_edge() {
    // @covers: create
    let a = NoopGrpcIngress::create();
    let b = NoopGrpcIngress::create();
    assert!(
        !Arc::ptr_eq(&a, &b),
        "successive create() calls must produce distinct instances"
    );
}
