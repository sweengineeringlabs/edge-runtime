//! Integration tests for GrpcServerConfigError.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_runtime_grpc::GrpcServerConfigError;

#[test]
fn test_grpc_server_config_error_tls_required_but_missing_has_descriptive_message() {
    let e = GrpcServerConfigError::TlsRequiredButMissing;
    assert!(e.to_string().contains("tls_required"));
}
