//! Conversion operations between gRPC status code representations.

use swe_edge_ingress_grpc::{GrpcIngressError, GrpcStatusCode};

/// Conversion operations between gRPC status code representations.
///
/// All methods are static and gated on `where Self: Sized`; [`StatusCodeConvert::svc_marker`]
/// keeps the trait object-safe.
pub trait StatusCodeConvert {
    /// Convert a [`tonic::Code`] into the crate-local [`GrpcStatusCode`].
    fn from_tonic_code(code: tonic::Code) -> GrpcStatusCode
    where
        Self: Sized;
    /// Convert a crate-local [`GrpcStatusCode`] into a [`tonic::Code`].
    fn to_tonic_code(code: GrpcStatusCode) -> tonic::Code
    where
        Self: Sized;
    /// Encode a [`GrpcStatusCode`] as the numeric `grpc-status` wire value.
    fn to_wire(code: GrpcStatusCode) -> i32
    where
        Self: Sized;
    /// Parse a numeric `grpc-status` wire value into a [`GrpcStatusCode`].
    fn from_wire(value: i32) -> GrpcStatusCode
    where
        Self: Sized;
    /// Map a [`GrpcIngressError`] to `(tonic::Code, on-wire message)`.
    fn map_inbound_error(e: GrpcIngressError) -> (tonic::Code, String)
    where
        Self: Sized;
    /// Object-safe marker method.
    fn svc_marker(&self) -> bool {
        true
    }
}
