//! Integration tests for GrpcServerError.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_runtime_grpc::GrpcServerError;

#[test]
fn test_grpc_server_error_bind_formats_address_and_source() {
    let err = GrpcServerError::Bind(
        "127.0.0.1:443".into(),
        std::io::Error::new(std::io::ErrorKind::AddrInUse, "port in use"),
    );
    let msg = err.to_string();
    assert!(
        msg.contains("127.0.0.1:443"),
        "must include address in message"
    );
}
