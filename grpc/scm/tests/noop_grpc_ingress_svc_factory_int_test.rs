//! Integration tests for NoopGrpcIngress factory methods.
#![allow(clippy::unwrap_used)]

use std::sync::Arc;

use swe_edge_runtime_grpc::NoopGrpcIngress;

// ── create ──────────────────────────────────────────────────────────────────

#[test]
fn test_create_returns_arc_with_strong_count_one_happy() {
    // @covers: create
    let arc = NoopGrpcIngress::create();
    assert_eq!(
        Arc::strong_count(&arc),
        1,
        "fresh Arc must have exactly one strong reference"
    );
}

#[test]
fn test_create_clone_increments_strong_count_error() {
    // @covers: create
    let arc = NoopGrpcIngress::create();
    let _clone = Arc::clone(&arc);
    assert_eq!(
        Arc::strong_count(&arc),
        2,
        "clone must increment strong count to 2"
    );
}

#[test]
fn test_create_two_calls_produce_independent_instances_edge() {
    // @covers: create
    let a = NoopGrpcIngress::create();
    let b = NoopGrpcIngress::create();
    assert_eq!(Arc::strong_count(&a), 1);
    assert_eq!(Arc::strong_count(&b), 1);
    assert!(
        !Arc::ptr_eq(&a, &b),
        "separate create() calls must produce distinct instances"
    );
}
