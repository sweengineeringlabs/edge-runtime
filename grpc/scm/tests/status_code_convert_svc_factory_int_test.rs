//! Integration tests for StatusCodeConvert factory methods and SAF surface.
#![allow(clippy::unwrap_used)]

use swe_edge_runtime_grpc::StatusCodeConvert;

#[test]
fn test_status_code_convert_svc_identifier_exists() {
    assert_eq!(
        swe_edge_runtime_grpc::STATUS_CODE_CONVERT_SVC,
        "status_code_convert"
    );
}

// ── svc_marker ────────────────────────────────────────────────────────────────

#[test]
fn test_svc_marker_returns_true_happy() {
    // @covers: svc_marker
    struct TestMarker;
    impl StatusCodeConvert for TestMarker {
        fn to_wire(_: swe_edge_ingress_grpc::GrpcStatusCode) -> i32 {
            0
        }
        fn from_wire(_: i32) -> swe_edge_ingress_grpc::GrpcStatusCode {
            swe_edge_ingress_grpc::GrpcStatusCode::Ok
        }
    }
    let t = TestMarker;
    assert!(t.svc_marker());
}

#[test]
fn test_svc_marker_always_true_error() {
    // @covers: svc_marker
    // Verifies svc_marker cannot return false.
    struct TestMarker;
    impl StatusCodeConvert for TestMarker {
        fn to_wire(_: swe_edge_ingress_grpc::GrpcStatusCode) -> i32 {
            0
        }
        fn from_wire(_: i32) -> swe_edge_ingress_grpc::GrpcStatusCode {
            swe_edge_ingress_grpc::GrpcStatusCode::Ok
        }
    }
    let t = TestMarker;
    assert_ne!(t.svc_marker(), false, "svc_marker must return true");
}

#[test]
fn test_svc_marker_consistent_edge() {
    // @covers: svc_marker
    // Verifies marker returns true consistently.
    struct TestMarker;
    impl StatusCodeConvert for TestMarker {
        fn to_wire(_: swe_edge_ingress_grpc::GrpcStatusCode) -> i32 {
            0
        }
        fn from_wire(_: i32) -> swe_edge_ingress_grpc::GrpcStatusCode {
            swe_edge_ingress_grpc::GrpcStatusCode::Ok
        }
    }
    let t = TestMarker;
    let first_call = t.svc_marker();
    let second_call = t.svc_marker();
    assert!(first_call && second_call);
}
