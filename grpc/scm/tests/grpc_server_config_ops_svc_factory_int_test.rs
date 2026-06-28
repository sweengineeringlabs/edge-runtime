//! Integration tests for GrpcServerConfigOps SAF factory.
#![allow(clippy::unwrap_used)]

#[test]
fn test_grpc_server_config_ops_svc_identifier_exists() {
    assert_eq!(
        swe_edge_runtime_grpc::GRPC_SERVER_CONFIG_OPS_SVC,
        "grpc_server_config_ops"
    );
}
