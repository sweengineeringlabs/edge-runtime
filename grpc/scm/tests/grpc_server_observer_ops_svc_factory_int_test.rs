//! Integration tests for GrpcServerObserverOps factory methods and SAF surface.
#![allow(clippy::unwrap_used)]

use swe_edge_runtime_grpc::GrpcServerObserverOps;

#[test]
fn test_grpc_server_observer_ops_svc_identifier_exists() {
    assert_eq!(
        swe_edge_runtime_grpc::GRPC_SERVER_OBSERVER_OPS_SVC,
        "grpc_server_observer_ops"
    );
}

// ── svc_marker ────────────────────────────────────────────────────────────────

#[test]
fn test_svc_marker_returns_true_happy() {
    // @covers: svc_marker
    struct TestMarker;
    impl GrpcServerObserverOps for TestMarker {}
    let t = TestMarker;
    assert!(t.svc_marker());
}

#[test]
fn test_svc_marker_always_true_error() {
    // @covers: svc_marker
    // Verifies svc_marker cannot return false.
    struct TestMarker;
    impl GrpcServerObserverOps for TestMarker {}
    let t = TestMarker;
    assert_ne!(t.svc_marker(), false, "svc_marker must return true");
}

#[test]
fn test_svc_marker_consistent_edge() {
    // @covers: svc_marker
    // Verifies marker returns true consistently.
    struct TestMarker;
    impl GrpcServerObserverOps for TestMarker {}
    let t = TestMarker;
    let first_call = t.svc_marker();
    let second_call = t.svc_marker();
    assert!(first_call && second_call);
}
