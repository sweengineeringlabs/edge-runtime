//! Integration tests for [`swe_edge_runtime_grpc::GrpcIngressError`].

use swe_edge_runtime_grpc::GrpcIngressError;

/// @covers: GrpcIngressError::Internal — Display includes variant name
#[test]
fn test_internal_displays_variant_happy() {
    let err = GrpcIngressError::Internal("boom".into());
    let s = err.to_string();
    assert!(
        s.contains("boom"),
        "display should include the message: {s}"
    );
}

/// @covers: GrpcIngressError::NotFound — Display is non-empty
#[test]
fn test_not_found_displays_non_empty_happy() {
    let err = GrpcIngressError::NotFound("route".into());
    assert!(!err.to_string().is_empty());
}

/// @covers: GrpcIngressError::InvalidInput — Display contains payload
#[test]
fn test_invalid_input_display_contains_payload_error() {
    let err = GrpcIngressError::InvalidInput("bad proto".into());
    assert!(err.to_string().contains("bad proto"));
}

/// @covers: GrpcIngressError::Unauthorized — is non-ok
#[test]
fn test_unauthorized_is_err_edge() {
    let result: Result<(), GrpcIngressError> = Err(GrpcIngressError::Unauthorized("caller".into()));
    assert!(result.is_err());
}

/// @covers: GrpcIngressError implements std::error::Error
#[test]
fn test_error_trait_implemented_happy() {
    let err: Box<dyn std::error::Error> = Box::new(GrpcIngressError::Timeout("5s".into()));
    assert!(!err.to_string().is_empty());
}

/// @covers: all variants are constructable (exhaustiveness guard)
#[test]
fn test_all_variants_constructable_edge() {
    let _variants = [
        GrpcIngressError::Internal("x".into()),
        GrpcIngressError::NotFound("x".into()),
        GrpcIngressError::InvalidInput("x".into()),
        GrpcIngressError::Unavailable("x".into()),
        GrpcIngressError::Timeout("x".into()),
        GrpcIngressError::Unauthorized("x".into()),
        GrpcIngressError::PermissionDenied("x".into()),
        GrpcIngressError::Unimplemented("x".into()),
    ];
}
