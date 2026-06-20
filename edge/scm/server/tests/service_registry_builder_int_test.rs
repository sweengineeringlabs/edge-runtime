//! Integration tests for [`ServiceRegistryBuilder`].

use std::sync::Arc;
use swe_edge_egress_http::HttpTransportSvc;
use swe_edge_runtime::ServiceRegistryBuilder;

fn make_http() -> Arc<dyn swe_edge_egress_http::HttpEgress> {
    Arc::from(
        HttpTransportSvc::default_http_egress().unwrap_or_else(|e| panic!("http egress: {e}")),
    )
}

// @covers: ServiceRegistryBuilder::new
// @covers: ServiceRegistryBuilder::build
#[test]
fn test_service_registry_builder_new_build_happy() {
    let reg = ServiceRegistryBuilder::new(make_http()).build();
    assert!(reg.grpc().is_none());
}

// @covers: ServiceRegistryBuilder::build (no egress_grpc)
#[test]
fn test_service_registry_builder_without_grpc_returns_none_edge() {
    let reg = ServiceRegistryBuilder::new(make_http()).build();
    assert!(reg.grpc().is_none());
}

// @covers: ServiceRegistryBuilder is exported from swe_edge_runtime SAF surface
#[test]
fn test_service_registry_builder_is_exported_from_runtime() {
    let _b: ServiceRegistryBuilder = ServiceRegistryBuilder::new(make_http());
}

// @covers: ServiceRegistryBuilder::build — http accessor survives round-trip
#[test]
fn test_service_registry_builder_http_accessor_happy() {
    let reg = ServiceRegistryBuilder::new(make_http()).build();
    let _http = reg.http();
}

// @covers: ServiceRegistryBuilder::grpc
#[test]
fn test_service_registry_builder_with_grpc_stores_none_without_client_edge() {
    let reg = ServiceRegistryBuilder::new(make_http()).build();
    assert!(reg.grpc().is_none(), "no grpc set via builder");
}

#[cfg(feature = "cli")]
// @covers: ServiceRegistryBuilder::cli_runner
#[test]
fn test_service_registry_builder_with_cli_runner_stores_runner_happy() {
    use swe_edge_runtime::NoopCliRunner;

    let runner = NoopCliRunner::create();
    let reg = ServiceRegistryBuilder::new(make_http())
        .cli_runner(Arc::new(runner))
        .build();
    assert!(reg.cli_runner().is_some());
}
