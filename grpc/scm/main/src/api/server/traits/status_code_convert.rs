//! Wire-level conversion operations between gRPC status code representations.

use swe_edge_ingress_grpc::GrpcStatusCode;

/// Wire-level conversion operations for gRPC status codes.
///
/// Transport-specific conversions (to/from `tonic::Code`) live as inherent
/// methods on the concrete [`StatusCodeConverter`](crate::api::StatusCodeConverter)
/// in spi/ so the api/ contract stays free of foreign transport types.
///
/// All methods are static and gated on `where Self: Sized`; [`StatusCodeConvert::svc_marker`]
/// keeps the trait object-safe.
pub trait StatusCodeConvert {
    /// Encode a [`GrpcStatusCode`] as the numeric `grpc-status` wire value.
    fn to_wire(code: GrpcStatusCode) -> i32
    where
        Self: Sized;
    /// Parse a numeric `grpc-status` wire value into a [`GrpcStatusCode`].
    fn from_wire(value: i32) -> GrpcStatusCode
    where
        Self: Sized;
    /// Object-safe marker method.
    fn svc_marker(&self) -> bool {
        true
    }
}
