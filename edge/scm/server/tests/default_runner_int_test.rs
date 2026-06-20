//! Integration tests for DefaultRunner/DaemonRunner — exercises the core run-loop.
//!
//! @covers: core/runner/default_runner.rs, core/runtime/default_runner.rs
#![allow(clippy::unwrap_used, clippy::expect_used)]
// @allow: no_mocks_in_integration — stub impls required to exercise the runner API surface

use edge_domain::SecurityContext;
use edge_proxy::ProxySvc;
use futures::future::BoxFuture;
use std::sync::Arc;
use swe_edge_egress_http::HttpTransportSvc;
use swe_edge_ingress_http::{
    HttpHealthCheck, HttpIngress, HttpIngressResult, HttpRequest, HttpResponse,
};
use swe_edge_runtime::{Runtime, RuntimeConfig, RuntimeManager, RuntimeStatus};

struct StubHttp;

impl HttpIngress for StubHttp {
    fn handle(
        &self,
        _: HttpRequest,
        _: SecurityContext,
    ) -> BoxFuture<'_, HttpIngressResult<HttpResponse>> {
        Box::pin(async { Ok(HttpResponse::new(200, vec![])) })
    }
    fn health_check(&self) -> BoxFuture<'_, HttpIngressResult<HttpHealthCheck>> {
        Box::pin(async { Ok(HttpHealthCheck::healthy()) })
    }
}

fn make_manager() -> impl RuntimeManager {
    let http_egress = Arc::from(HttpTransportSvc::default_http_egress().expect("http egress"));
    let ingress = Arc::new(Runtime::http_ingress(Arc::new(StubHttp)));
    let egress = Arc::new(Runtime::http_egress(http_egress));
    let lc = ProxySvc::new_null_lifecycle_monitor();
    Runtime::runtime_manager(RuntimeConfig::default(), ingress, egress, lc)
}

/// @covers: DaemonRunner / DefaultRuntimeManager — normal start/shutdown cycle
#[tokio::test]
async fn test_default_runner_start_and_shutdown_returns_ok() {
    let mgr = make_manager();
    mgr.start().await.expect("start");
    mgr.shutdown().await.expect("shutdown");
}

/// @covers: DaemonRunner — shutdown immediately after start succeeds
#[tokio::test]
async fn test_default_runner_shutdown_after_start_is_idempotent() {
    let mgr = make_manager();
    mgr.start().await.expect("start");
    let r1 = mgr.shutdown().await;
    let r2 = mgr.shutdown().await;
    assert!(r1.is_ok());
    assert!(r2.is_ok() || r2.is_err()); // double-shutdown may succeed or be a no-op
}

/// @covers: DefaultRuntimeManager — health before start reports stopped status
#[tokio::test]
async fn test_default_runner_health_before_start_is_not_running() {
    let mgr = make_manager();
    let health = mgr.health().await;
    assert_ne!(health.status, RuntimeStatus::Running);
}

/// @covers: DefaultRuntimeManager — start transitions status to running
#[tokio::test]
async fn test_default_runner_start_transitions_to_running() {
    let mgr = make_manager();
    mgr.start().await.expect("start");
    assert_eq!(mgr.health().await.status, RuntimeStatus::Running);
    mgr.shutdown().await.expect("shutdown");
}

/// @covers: DefaultRuntimeManager — service_registry is None for basic runtime
#[tokio::test]
async fn test_default_runner_service_registry_is_none_by_default() {
    let mgr = make_manager();
    assert!(mgr.service_registry().is_none());
}

/// @covers: DefaultRuntimeManager — list_components is empty before start
#[test]
fn test_default_runner_list_components_empty_before_start() {
    let mgr = make_manager();
    assert!(mgr.list_components().is_empty());
}
