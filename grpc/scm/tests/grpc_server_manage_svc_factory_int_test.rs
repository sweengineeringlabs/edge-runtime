//! Integration tests for GrpcServerManage SAF factory.
#![allow(clippy::unwrap_used)]

#[test]
fn test_grpc_server_manage_svc_identifier_exists() {
    assert_eq!(
        swe_edge_runtime_grpc::GRPC_SERVER_MANAGE_SVC,
        "grpc_server_manage"
    );
}
