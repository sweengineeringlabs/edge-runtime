//! StatusCodeConverter — converter type declaration and inherent impl.

use swe_edge_ingress_grpc::{GrpcIngressError, GrpcStatusCode};

use crate::api::SANITIZED_INTERNAL_MSG;

/// Converter for gRPC status codes between representations.
pub struct StatusCodeConverter;

impl StatusCodeConverter {
    /// Convert a [`tonic::Code`] into the crate-local [`GrpcStatusCode`].  Total.
    pub fn from_tonic_code(code: tonic::Code) -> GrpcStatusCode {
        match code {
            tonic::Code::Ok => GrpcStatusCode::Ok,
            tonic::Code::Cancelled => GrpcStatusCode::Cancelled,
            tonic::Code::Unknown => GrpcStatusCode::Unknown,
            tonic::Code::InvalidArgument => GrpcStatusCode::InvalidArgument,
            tonic::Code::DeadlineExceeded => GrpcStatusCode::DeadlineExceeded,
            tonic::Code::NotFound => GrpcStatusCode::NotFound,
            tonic::Code::AlreadyExists => GrpcStatusCode::AlreadyExists,
            tonic::Code::PermissionDenied => GrpcStatusCode::PermissionDenied,
            tonic::Code::ResourceExhausted => GrpcStatusCode::ResourceExhausted,
            tonic::Code::FailedPrecondition => GrpcStatusCode::FailedPrecondition,
            tonic::Code::Aborted => GrpcStatusCode::Aborted,
            tonic::Code::OutOfRange => GrpcStatusCode::OutOfRange,
            tonic::Code::Unimplemented => GrpcStatusCode::Unimplemented,
            tonic::Code::Internal => GrpcStatusCode::Internal,
            tonic::Code::Unavailable => GrpcStatusCode::Unavailable,
            tonic::Code::DataLoss => GrpcStatusCode::DataLoss,
            tonic::Code::Unauthenticated => GrpcStatusCode::Unauthenticated,
        }
    }

    /// Convert a crate-local [`GrpcStatusCode`] into a [`tonic::Code`].  Total.
    pub fn to_tonic_code(code: GrpcStatusCode) -> tonic::Code {
        match code {
            GrpcStatusCode::Ok => tonic::Code::Ok,
            GrpcStatusCode::Cancelled => tonic::Code::Cancelled,
            GrpcStatusCode::Unknown => tonic::Code::Unknown,
            GrpcStatusCode::InvalidArgument => tonic::Code::InvalidArgument,
            GrpcStatusCode::DeadlineExceeded => tonic::Code::DeadlineExceeded,
            GrpcStatusCode::NotFound => tonic::Code::NotFound,
            GrpcStatusCode::AlreadyExists => tonic::Code::AlreadyExists,
            GrpcStatusCode::PermissionDenied => tonic::Code::PermissionDenied,
            GrpcStatusCode::ResourceExhausted => tonic::Code::ResourceExhausted,
            GrpcStatusCode::FailedPrecondition => tonic::Code::FailedPrecondition,
            GrpcStatusCode::Aborted => tonic::Code::Aborted,
            GrpcStatusCode::OutOfRange => tonic::Code::OutOfRange,
            GrpcStatusCode::Unimplemented => tonic::Code::Unimplemented,
            GrpcStatusCode::Internal => tonic::Code::Internal,
            GrpcStatusCode::Unavailable => tonic::Code::Unavailable,
            GrpcStatusCode::DataLoss => tonic::Code::DataLoss,
            GrpcStatusCode::Unauthenticated => tonic::Code::Unauthenticated,
        }
    }

    /// Encode a [`GrpcStatusCode`] as the numeric `grpc-status` wire value.
    pub fn to_wire(code: GrpcStatusCode) -> i32 {
        Self::to_tonic_code(code) as i32
    }

    /// Parse a numeric `grpc-status` wire value into a [`GrpcStatusCode`].
    ///
    /// Returns `Unknown` for unrecognized values per the gRPC spec.
    pub fn from_wire(value: i32) -> GrpcStatusCode {
        Self::from_tonic_code(tonic::Code::from(value))
    }

    /// Map a [`GrpcIngressError`] to `(tonic::Code, on-wire message)`.
    pub fn map_inbound_error(e: GrpcIngressError) -> (tonic::Code, String) {
        match e {
            GrpcIngressError::Status(code, msg) => (Self::to_tonic_code(code), msg),
            GrpcIngressError::Internal(msg) => {
                tracing::warn!(server_internal_msg = %msg, "gRPC handler returned Internal — sanitizing for wire");
                (tonic::Code::Internal, SANITIZED_INTERNAL_MSG.to_owned())
            }
            GrpcIngressError::NotFound(m) => (tonic::Code::NotFound, m),
            GrpcIngressError::InvalidArgument(m) => (tonic::Code::InvalidArgument, m),
            GrpcIngressError::Unavailable(m) => (tonic::Code::Unavailable, m),
            GrpcIngressError::DeadlineExceeded(m) => (tonic::Code::DeadlineExceeded, m),
            GrpcIngressError::PermissionDenied(m) => (tonic::Code::PermissionDenied, m),
            GrpcIngressError::Unimplemented(m) => (tonic::Code::Unimplemented, m),
        }
    }
}
