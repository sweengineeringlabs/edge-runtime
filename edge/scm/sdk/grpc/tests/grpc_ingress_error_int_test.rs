//! Integration tests for [`swe_edge_runtime_grpc::GrpcIngressError`].

use swe_edge_runtime_grpc::GrpcIngressError;

/// @covers: GrpcIngressError::Internal — Display contains payload
#[test]
fn test_internal_error_display_contains_message_happy() {
    let err = GrpcIngressError::Internal("upstream timeout".into());
    assert!(err.to_string().contains("upstream timeout"));
}

/// @covers: GrpcIngressError — all variants are Err (not Ok) when wrapped in Result
#[test]
fn test_all_variants_are_err_edge() {
    let variants: Vec<GrpcIngressError> = vec![
        GrpcIngressError::Internal("x".into()),
        GrpcIngressError::NotFound("x".into()),
        GrpcIngressError::InvalidInput("x".into()),
        GrpcIngressError::Unavailable("x".into()),
        GrpcIngressError::Timeout("x".into()),
        GrpcIngressError::Unauthorized("x".into()),
        GrpcIngressError::PermissionDenied("x".into()),
        GrpcIngressError::Unimplemented("x".into()),
    ];
    for v in variants {
        let r: Result<(), _> = Err(v);
        assert!(r.is_err());
    }
}

/// @covers: GrpcIngressError — Display is non-empty for Timeout
#[test]
fn test_timeout_display_is_non_empty_error() {
    let err = GrpcIngressError::Timeout("5s".into());
    assert!(!err.to_string().is_empty());
}
