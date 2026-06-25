//! Integration tests for StatusCodeConverter factory methods.
#![allow(clippy::unwrap_used)]

use swe_edge_ingress_grpc::{GrpcIngressError, GrpcStatusCode};
use swe_edge_runtime_grpc::{StatusCodeConverter, SANITIZED_INTERNAL_MSG};

// ── from_tonic_code ─────────────────────────────────────────────────────────

#[test]
fn test_from_tonic_code_ok_maps_to_ok_happy() {
    // @covers: from_tonic_code
    assert_eq!(StatusCodeConverter::from_tonic_code(tonic::Code::Ok), GrpcStatusCode::Ok);
}

#[test]
fn test_from_tonic_code_not_found_maps_correctly_happy() {
    // @covers: from_tonic_code
    assert_eq!(
        StatusCodeConverter::from_tonic_code(tonic::Code::NotFound),
        GrpcStatusCode::NotFound
    );
    assert_ne!(
        StatusCodeConverter::from_tonic_code(tonic::Code::NotFound),
        GrpcStatusCode::Ok
    );
}

#[test]
fn test_from_tonic_code_internal_maps_to_internal_error() {
    // @covers: from_tonic_code
    let code = StatusCodeConverter::from_tonic_code(tonic::Code::Internal);
    assert_eq!(code, GrpcStatusCode::Internal);
    assert_ne!(code, GrpcStatusCode::Ok, "Internal must not map to Ok");
}

#[test]
fn test_from_tonic_code_unknown_maps_to_non_ok_edge() {
    // @covers: from_tonic_code
    let code = StatusCodeConverter::from_tonic_code(tonic::Code::Unknown);
    assert_ne!(code, GrpcStatusCode::Ok, "Unknown must not map to Ok");
}

// ── to_tonic_code ───────────────────────────────────────────────────────────

#[test]
fn test_to_tonic_code_not_found_maps_correctly_happy() {
    // @covers: to_tonic_code
    assert_eq!(
        StatusCodeConverter::to_tonic_code(GrpcStatusCode::NotFound),
        tonic::Code::NotFound
    );
}

#[test]
fn test_to_tonic_code_ok_maps_to_ok_happy() {
    // @covers: to_tonic_code
    assert_eq!(StatusCodeConverter::to_tonic_code(GrpcStatusCode::Ok), tonic::Code::Ok);
    assert_ne!(StatusCodeConverter::to_tonic_code(GrpcStatusCode::Ok), tonic::Code::NotFound);
}

#[test]
fn test_to_tonic_code_internal_maps_to_internal_error() {
    // @covers: to_tonic_code
    let code = StatusCodeConverter::to_tonic_code(GrpcStatusCode::Internal);
    assert_eq!(code, tonic::Code::Internal);
    assert_ne!(code, tonic::Code::Ok, "Internal must not map to Ok");
}

#[test]
fn test_to_tonic_code_roundtrip_consistency_edge() {
    // @covers: to_tonic_code
    let tonic_code = tonic::Code::PermissionDenied;
    let grpc_code = StatusCodeConverter::from_tonic_code(tonic_code);
    let back = StatusCodeConverter::to_tonic_code(grpc_code);
    assert_eq!(back, tonic_code, "round-trip must preserve code identity");
}

// ── to_wire ─────────────────────────────────────────────────────────────────

#[test]
fn test_to_wire_ok_is_zero_happy() {
    // @covers: to_wire
    assert_eq!(StatusCodeConverter::to_wire(GrpcStatusCode::Ok), 0);
}

#[test]
fn test_to_wire_not_found_is_nonzero_error() {
    // @covers: to_wire
    let wire = StatusCodeConverter::to_wire(GrpcStatusCode::NotFound);
    assert_ne!(wire, 0, "NotFound must not map to 0 (which is Ok)");
}

#[test]
fn test_to_wire_different_codes_produce_different_values_edge() {
    // @covers: to_wire
    let ok = StatusCodeConverter::to_wire(GrpcStatusCode::Ok);
    let not_found = StatusCodeConverter::to_wire(GrpcStatusCode::NotFound);
    assert_ne!(ok, not_found, "different status codes must produce different wire values");
}

// ── from_wire ───────────────────────────────────────────────────────────────

#[test]
fn test_from_wire_zero_is_ok_happy() {
    // @covers: from_wire
    assert_eq!(StatusCodeConverter::from_wire(0), GrpcStatusCode::Ok);
}

#[test]
fn test_from_wire_roundtrip_not_found_edge() {
    // @covers: from_wire
    let wire = StatusCodeConverter::to_wire(GrpcStatusCode::NotFound);
    assert_eq!(StatusCodeConverter::from_wire(wire), GrpcStatusCode::NotFound);
}

#[test]
fn test_from_wire_nonzero_is_not_ok_error() {
    // @covers: from_wire
    // Wire value 5 is PermissionDenied in gRPC protocol — must not map to Ok
    let wire = StatusCodeConverter::to_wire(GrpcStatusCode::PermissionDenied);
    assert_ne!(StatusCodeConverter::from_wire(wire), GrpcStatusCode::Ok, "non-zero wire must not map to Ok");
}

// ── map_inbound_error ───────────────────────────────────────────────────────

#[test]
fn test_map_inbound_error_status_passes_through_happy() {
    // @covers: map_inbound_error
    let (code, msg) = StatusCodeConverter::map_inbound_error(GrpcIngressError::Status(
        GrpcStatusCode::NotFound,
        "resource gone".into(),
    ));
    assert_eq!(code, tonic::Code::NotFound);
    assert_eq!(msg, "resource gone");
}

#[test]
fn test_map_inbound_error_internal_sanitizes_message_error() {
    // @covers: map_inbound_error
    let (code, msg) = StatusCodeConverter::map_inbound_error(GrpcIngressError::Internal(
        "secret db password".into(),
    ));
    assert_eq!(code, tonic::Code::Internal, "Internal error must map to tonic::Code::Internal");
    assert_eq!(msg, SANITIZED_INTERNAL_MSG, "Internal error must sanitize the wire message");
    assert!(!msg.contains("secret"), "sanitized message must not leak internal details");
}

#[test]
fn test_map_inbound_error_not_found_variant_edge() {
    // @covers: map_inbound_error
    let (code, _msg) = StatusCodeConverter::map_inbound_error(GrpcIngressError::NotFound(
        "item not found".into(),
    ));
    assert_eq!(code, tonic::Code::NotFound);
}
