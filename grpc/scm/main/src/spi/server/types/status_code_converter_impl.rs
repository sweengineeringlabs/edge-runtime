//! StatusCodeConvert trait impl + tonic-specific inherent conversions for StatusCodeConverter.
//!
//! `tonic::Code` is a transport detail and must not appear in the api/ contract,
//! so the to/from-tonic conversions live here in spi/ as inherent methods. The
//! api/ [`StatusCodeConvert`] trait exposes only wire-level (`i32`) operations.
use swe_edge_ingress_grpc::{GrpcIngressError, GrpcStatusCode};

use crate::api::{StatusCodeConvert, StatusCodeConverter, SANITIZED_INTERNAL_MSG};

impl StatusCodeConverter {
    /// Convert a [`tonic::Code`] into the crate-local [`GrpcStatusCode`].
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

    /// Convert a crate-local [`GrpcStatusCode`] into a [`tonic::Code`].
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

impl StatusCodeConvert for StatusCodeConverter {
    fn to_wire(code: GrpcStatusCode) -> i32 {
        Self::to_tonic_code(code) as i32
    }

    fn from_wire(value: i32) -> GrpcStatusCode {
        Self::from_tonic_code(tonic::Code::from(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use swe_edge_ingress_grpc::{GrpcIngressError, GrpcStatusCode};

    /// @covers: from_tonic_code
    #[test]
    fn test_from_tonic_code_ok_maps_to_ok_happy() {
        assert_eq!(
            StatusCodeConverter::from_tonic_code(tonic::Code::Ok),
            GrpcStatusCode::Ok
        );
    }

    /// @covers: from_tonic_code
    #[test]
    fn test_from_tonic_code_unknown_maps_to_unknown_error() {
        assert_eq!(
            StatusCodeConverter::from_tonic_code(tonic::Code::Unknown),
            GrpcStatusCode::Unknown
        );
    }

    /// @covers: from_tonic_code
    #[test]
    fn test_from_tonic_code_unauthenticated_maps_correctly_edge() {
        assert_eq!(
            StatusCodeConverter::from_tonic_code(tonic::Code::Unauthenticated),
            GrpcStatusCode::Unauthenticated
        );
    }

    /// @covers: to_tonic_code
    #[test]
    fn test_to_tonic_code_ok_maps_to_ok_happy() {
        assert_eq!(
            StatusCodeConverter::to_tonic_code(GrpcStatusCode::Ok),
            tonic::Code::Ok
        );
    }

    /// @covers: to_tonic_code
    #[test]
    fn test_to_tonic_code_internal_maps_to_internal_error() {
        assert_eq!(
            StatusCodeConverter::to_tonic_code(GrpcStatusCode::Internal),
            tonic::Code::Internal
        );
    }

    /// @covers: to_tonic_code
    #[test]
    fn test_to_tonic_code_unauthenticated_maps_correctly_edge() {
        assert_eq!(
            StatusCodeConverter::to_tonic_code(GrpcStatusCode::Unauthenticated),
            tonic::Code::Unauthenticated
        );
    }

    /// @covers: map_inbound_error
    #[test]
    fn test_map_inbound_error_not_found_maps_to_not_found_happy() {
        let (code, msg) =
            StatusCodeConverter::map_inbound_error(GrpcIngressError::NotFound("x".into()));
        assert_eq!(code, tonic::Code::NotFound);
        assert_eq!(msg, "x");
    }

    /// @covers: map_inbound_error
    #[test]
    fn test_map_inbound_error_internal_sanitizes_message_error() {
        let (code, msg) =
            StatusCodeConverter::map_inbound_error(GrpcIngressError::Internal("secret".into()));
        assert_eq!(code, tonic::Code::Internal);
        assert_ne!(msg, "secret", "internal message must be sanitized");
    }

    /// @covers: map_inbound_error
    #[test]
    fn test_map_inbound_error_status_preserves_code_and_msg_edge() {
        let (code, msg) = StatusCodeConverter::map_inbound_error(GrpcIngressError::Status(
            GrpcStatusCode::Unimplemented,
            "not yet".into(),
        ));
        assert_eq!(code, tonic::Code::Unimplemented);
        assert_eq!(msg, "not yet");
    }
}
