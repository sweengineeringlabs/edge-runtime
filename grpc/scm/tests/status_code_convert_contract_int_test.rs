//! Contract test for the api/ `StatusCodeConvert` trait.
//!
//! Proves the public contract is implementable by an independent test double
//! (an encoding deliberately different from `StatusCodeConverter`'s tonic-based
//! mapping), exercised through the external consumer import path.
#![allow(clippy::unwrap_used)]

use swe_edge_ingress_grpc::GrpcStatusCode;
use swe_edge_runtime_grpc::StatusCodeConvert;

/// Alternate encoding: Ok ↔ 0, everything else ↔ a single sentinel. This is a
/// valid-but-different implementation, so passing tests prove `StatusCodeConvert`
/// is a genuine contract rather than behaviour tied to one concrete type.
struct ContractDouble;
impl StatusCodeConvert for ContractDouble {
    fn to_wire(code: GrpcStatusCode) -> i32 {
        match code {
            GrpcStatusCode::Ok => 0,
            _ => 2,
        }
    }
    fn from_wire(value: i32) -> GrpcStatusCode {
        match value {
            0 => GrpcStatusCode::Ok,
            _ => GrpcStatusCode::Unknown,
        }
    }
}

/// @covers: to_wire
#[test]
fn test_to_wire_double_encodes_ok_as_zero_happy() {
    assert_eq!(ContractDouble::to_wire(GrpcStatusCode::Ok), 0);
}

/// @covers: to_wire
#[test]
fn test_to_wire_double_encodes_non_ok_as_sentinel_error() {
    assert_eq!(ContractDouble::to_wire(GrpcStatusCode::Internal), 2);
}

/// @covers: from_wire
#[test]
fn test_from_wire_double_decodes_unknown_value_to_unknown_edge() {
    assert_eq!(ContractDouble::from_wire(42), GrpcStatusCode::Unknown);
}
