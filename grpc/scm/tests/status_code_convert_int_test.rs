//! Scenario coverage for the `StatusCodeConvert` trait's wire operations,
//! exercised through the concrete `StatusCodeConverter`.
#![allow(clippy::unwrap_used)]

use swe_edge_ingress_grpc::GrpcStatusCode;
use swe_edge_runtime_grpc::{StatusCodeConvert, StatusCodeConverter};

// ── to_wire ──────────────────────────────────────────────────────────────────

/// @covers: to_wire
#[test]
fn test_to_wire_ok_maps_to_zero_happy() {
    assert_eq!(StatusCodeConverter::to_wire(GrpcStatusCode::Ok), 0);
}

/// @covers: to_wire
#[test]
fn test_to_wire_internal_maps_to_thirteen_error() {
    // Internal is the canonical server-error status (grpc-status 13).
    assert_eq!(StatusCodeConverter::to_wire(GrpcStatusCode::Internal), 13);
}

/// @covers: to_wire
#[test]
fn test_to_wire_unauthenticated_is_highest_code_edge() {
    // Unauthenticated (16) is the largest standard grpc-status value.
    assert_eq!(
        StatusCodeConverter::to_wire(GrpcStatusCode::Unauthenticated),
        16
    );
}

// ── from_wire ────────────────────────────────────────────────────────────────

/// @covers: from_wire
#[test]
fn test_from_wire_zero_maps_to_ok_happy() {
    assert_eq!(StatusCodeConverter::from_wire(0), GrpcStatusCode::Ok);
}

/// @covers: from_wire
#[test]
fn test_from_wire_out_of_range_maps_to_unknown_error() {
    // Values above the defined range decode to Unknown rather than panicking.
    assert_eq!(StatusCodeConverter::from_wire(999), GrpcStatusCode::Unknown);
}

/// @covers: from_wire
#[test]
fn test_from_wire_negative_maps_to_unknown_edge() {
    // Negative wire values are invalid and decode to Unknown.
    assert_eq!(StatusCodeConverter::from_wire(-1), GrpcStatusCode::Unknown);
}

// ── round-trip ───────────────────────────────────────────────────────────────

/// @covers: to_wire
/// @covers: from_wire
#[test]
fn test_to_wire_then_from_wire_round_trips_all_variants_happy() {
    for code in [
        GrpcStatusCode::Ok,
        GrpcStatusCode::Cancelled,
        GrpcStatusCode::Internal,
        GrpcStatusCode::Unauthenticated,
    ] {
        let wire = StatusCodeConverter::to_wire(code);
        assert_eq!(StatusCodeConverter::from_wire(wire), code);
    }
}
