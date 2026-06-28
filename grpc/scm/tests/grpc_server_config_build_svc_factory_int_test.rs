//! Integration tests for GrpcServerConfigBuild SAF factory.
#![allow(clippy::unwrap_used)]

#[test]
fn test_grpc_server_config_build_svc_identifier_exists() {
    assert_eq!(
        swe_edge_runtime_grpc::GRPC_SERVER_CONFIG_BUILD_SVC,
        "grpc_server_config_build"
    );
}
