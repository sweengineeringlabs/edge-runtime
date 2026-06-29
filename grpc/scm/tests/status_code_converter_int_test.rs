//! Integration tests for StatusCodeConverter (api/server/types/status_code_converter.rs).
//! @covers: StatusCodeConverter::from_tonic_code
//! @covers: StatusCodeConverter::to_tonic_code
//! @covers: StatusCodeConverter::to_wire
//! @covers: StatusCodeConverter::from_wire
//! @covers: StatusCodeConverter::map_inbound_error
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_runtime_grpc::{
    GrpcIngressError, GrpcStatusCode, StatusCodeConvert, StatusCodeConverter,
    SANITIZED_INTERNAL_MSG,
};

// ── from_tonic_code ───────────────────────────────────────────────────────────

#[test]
fn test_status_code_converter_from_tonic_code_ok_maps_to_ok_happy() {
    // @covers: StatusCodeConverter::from_tonic_code
    assert!(matches!(
        StatusCodeConverter::from_tonic_code(tonic::Code::Ok),
        GrpcStatusCode::Ok
    ));
}

#[test]
fn test_status_code_converter_from_tonic_code_internal_maps_to_internal_error() {
    // @covers: StatusCodeConverter::from_tonic_code
    // The error path: Internal must map to Internal, not OK.
    assert!(matches!(
        StatusCodeConverter::from_tonic_code(tonic::Code::Internal),
        GrpcStatusCode::Internal
    ));
}

#[test]
fn test_status_code_converter_from_tonic_code_unauthenticated_maps_correctly_edge() {
    // @covers: StatusCodeConverter::from_tonic_code
    // Boundary code Unauthenticated must round-trip.
    assert!(matches!(
        StatusCodeConverter::from_tonic_code(tonic::Code::Unauthenticated),
        GrpcStatusCode::Unauthenticated
    ));
}

// ── to_tonic_code ─────────────────────────────────────────────────────────────

#[test]
fn test_status_code_converter_to_tonic_code_ok_maps_to_ok_happy() {
    // @covers: StatusCodeConverter::to_tonic_code
    assert!(matches!(
        StatusCodeConverter::to_tonic_code(GrpcStatusCode::Ok),
        tonic::Code::Ok
    ));
}

#[test]
fn test_status_code_converter_to_tonic_code_not_found_maps_correctly_error() {
    // @covers: StatusCodeConverter::to_tonic_code
    assert!(matches!(
        StatusCodeConverter::to_tonic_code(GrpcStatusCode::NotFound),
        tonic::Code::NotFound
    ));
}

#[test]
fn test_status_code_converter_to_tonic_code_data_loss_maps_edge() {
    // @covers: StatusCodeConverter::to_tonic_code
    assert!(matches!(
        StatusCodeConverter::to_tonic_code(GrpcStatusCode::DataLoss),
        tonic::Code::DataLoss
    ));
}

// ── to_wire / from_wire ───────────────────────────────────────────────────────

#[test]
fn test_status_code_converter_to_wire_ok_is_zero_happy() {
    // @covers: StatusCodeConverter::to_wire
    // gRPC spec: grpc-status: 0 = OK.
    assert_eq!(StatusCodeConverter::to_wire(GrpcStatusCode::Ok), 0);
}

#[test]
fn test_status_code_converter_from_wire_unknown_value_returns_unknown_error() {
    // @covers: StatusCodeConverter::from_wire
    // Out-of-range wire value → Unknown per gRPC spec.
    let code = StatusCodeConverter::from_wire(9999);
    assert!(matches!(code, GrpcStatusCode::Unknown));
}

#[test]
fn test_status_code_converter_to_wire_from_wire_round_trip_edge() {
    // @covers: StatusCodeConverter::to_wire
    // @covers: StatusCodeConverter::from_wire
    // Every well-known status code must survive an integer round-trip.
    let pairs = [
        (GrpcStatusCode::Ok, GrpcStatusCode::Ok),
        (GrpcStatusCode::Cancelled, GrpcStatusCode::Cancelled),
        (GrpcStatusCode::Internal, GrpcStatusCode::Internal),
        (
            GrpcStatusCode::Unauthenticated,
            GrpcStatusCode::Unauthenticated,
        ),
    ];
    for (code, expected) in pairs {
        let wire = StatusCodeConverter::to_wire(code);
        let back = StatusCodeConverter::from_wire(wire);
        assert!(
            matches!(
                (back, expected),
                (GrpcStatusCode::Ok, GrpcStatusCode::Ok)
                    | (GrpcStatusCode::Cancelled, GrpcStatusCode::Cancelled)
                    | (GrpcStatusCode::Internal, GrpcStatusCode::Internal)
                    | (
                        GrpcStatusCode::Unauthenticated,
                        GrpcStatusCode::Unauthenticated
                    )
            ),
            "round-trip failed"
        );
    }
}

// ── map_inbound_error ─────────────────────────────────────────────────────────

#[test]
fn test_status_code_converter_map_inbound_error_internal_sanitizes_message_happy() {
    // @covers: StatusCodeConverter::map_inbound_error
    // Internal errors must not leak raw messages to the wire.
    let (code, msg) = StatusCodeConverter::map_inbound_error(GrpcIngressError::Internal(
        "secret crash: null ptr 0x00".to_string(),
    ));
    assert!(matches!(code, tonic::Code::Internal));
    assert_eq!(
        msg, SANITIZED_INTERNAL_MSG,
        "raw message must be sanitized for wire"
    );
}

#[test]
fn test_status_code_converter_map_inbound_error_not_found_passes_through_error() {
    // @covers: StatusCodeConverter::map_inbound_error
    // Non-internal errors pass the message through unchanged.
    let (code, msg) = StatusCodeConverter::map_inbound_error(GrpcIngressError::NotFound(
        "user/42 not found".to_string(),
    ));
    assert!(matches!(code, tonic::Code::NotFound));
    assert_eq!(msg, "user/42 not found");
}

#[test]
fn test_status_code_converter_map_inbound_error_deadline_exceeded_maps_correctly_edge() {
    // @covers: StatusCodeConverter::map_inbound_error
    let (code, _msg) = StatusCodeConverter::map_inbound_error(GrpcIngressError::DeadlineExceeded(
        "timed out".to_string(),
    ));
    assert!(matches!(code, tonic::Code::DeadlineExceeded));
}
