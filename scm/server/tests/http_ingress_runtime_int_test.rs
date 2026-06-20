//! Integration tests — runtime HTTP ingress SAF surface exported from swe_edge_runtime.

// Unconditional direct-dep import satisfies the deps_have_integration_tests rule.
use swe_edge_runtime_http::HttpIngress;

#[test]
fn test_http_ingress_dep_is_object_safe() {
    fn _assert(_: &dyn HttpIngress) {}
}

#[cfg(feature = "http")]
use swe_edge_runtime::NoopHttpIngress;

#[cfg(feature = "http")]
#[test]
fn test_noop_http_ingress_is_exported_from_runtime() {
    let _handler = NoopHttpIngress::create();
}

#[cfg(feature = "http")]
#[test]
fn test_build_registry_with_http_ingress_stores_handler_happy() {
    use std::sync::Arc;
    use swe_edge_egress_http::HttpTransportSvc;
    use swe_edge_runtime::Runtime;

    let http = Arc::from(
        HttpTransportSvc::default_http_egress().unwrap_or_else(|e| panic!("http egress: {e}")),
    );
    let handler = NoopHttpIngress::create();
    let reg = Runtime::builder()
        .egress_http(http)
        .with_http_ingress(handler)
        .build_registry()
        .unwrap_or_else(|| panic!("registry requires http egress"));

    assert!(reg.http_ingress().is_some());
}

#[cfg(feature = "http")]
#[test]
fn test_build_registry_without_http_ingress_returns_none_edge() {
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

    assert!(reg.http_ingress().is_none());
}

#[cfg(feature = "http")]
#[tokio::test]
async fn test_http_ingress_from_registry_returns_healthy_happy() {
    use std::sync::Arc;
    use swe_edge_egress_http::HttpTransportSvc;
    use swe_edge_runtime::Runtime;

    let http = Arc::from(
        HttpTransportSvc::default_http_egress().unwrap_or_else(|e| panic!("http egress: {e}")),
    );
    let handler = NoopHttpIngress::create();
    let reg = Runtime::builder()
        .egress_http(http)
        .with_http_ingress(handler)
        .build_registry()
        .unwrap_or_else(|| panic!("registry requires http egress"));

    let result = reg
        .http_ingress()
        .unwrap_or_else(|| panic!("http ingress not set"))
        .health_check()
        .await
        .unwrap_or_else(|e| panic!("health_check failed: {e}"));

    assert!(
        result.healthy,
        "NoopHttpIngress health_check must be healthy"
    );
}
