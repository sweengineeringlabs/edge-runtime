//! Integration tests — runtime gRPC ingress SAF surface exported from swe_edge_runtime.

// Unconditional direct-dep import satisfies the deps_have_integration_tests rule.
use swe_edge_runtime_grpc::GrpcIngress;

#[test]
fn test_grpc_ingress_dep_is_object_safe() {
    fn _assert(_: &dyn GrpcIngress) {}
}

#[cfg(feature = "grpc")]
use swe_edge_runtime::NoopGrpcIngress;

#[cfg(feature = "grpc")]
#[test]
fn test_noop_grpc_ingress_is_exported_from_runtime() {
    let _handler = NoopGrpcIngress::create();
}

#[cfg(feature = "grpc")]
#[test]
fn test_build_registry_with_grpc_ingress_stores_handler_happy() {
    use std::sync::Arc;
    use swe_edge_egress_http::HttpTransportSvc;
    use swe_edge_runtime::Runtime;

    let http = Arc::from(
        HttpTransportSvc::default_http_egress().unwrap_or_else(|e| panic!("http egress: {e}")),
    );
    let handler = NoopGrpcIngress::create();
    let reg = Runtime::builder()
        .egress_http(http)
        .with_grpc_ingress(handler)
        .build_registry()
        .unwrap_or_else(|| panic!("registry requires http egress"));

    assert!(reg.grpc_ingress().is_some());
}

#[cfg(feature = "grpc")]
#[test]
fn test_build_registry_without_grpc_ingress_returns_none_edge() {
    use std::sync::Arc;
    use swe_edge_egress_http::HttpTransportSvc;
    use swe_edge_runtime::Runtime;

    let http = Arc::from(
        HttpTransportSvc::default_http_egress().unwrap_or_else(|e| panic!("http egress: {e}")),
    );
    let reg = Runtime::builder()
        .egress_http(http)
        .build_registry()
        .unwrap_or_else(|| panic!("registry requires http egress"));

    assert!(reg.grpc_ingress().is_none());
}

#[cfg(feature = "grpc")]
#[tokio::test]
async fn test_grpc_ingress_from_registry_returns_healthy_happy() {
    use std::sync::Arc;
    use swe_edge_egress_http::HttpTransportSvc;
    use swe_edge_runtime::Runtime;

    let http = Arc::from(
        HttpTransportSvc::default_http_egress().unwrap_or_else(|e| panic!("http egress: {e}")),
    );
    let handler = NoopGrpcIngress::create();
    let reg = Runtime::builder()
        .egress_http(http)
        .with_grpc_ingress(handler)
        .build_registry()
        .unwrap_or_else(|| panic!("registry requires http egress"));

    let result = reg
        .grpc_ingress()
        .unwrap_or_else(|| panic!("grpc ingress not set"))
        .health_check()
        .await
        .unwrap_or_else(|e| panic!("health_check failed: {e}"));

    assert!(
        result.healthy,
        "NoopGrpcIngress health_check must be healthy"
    );
}
