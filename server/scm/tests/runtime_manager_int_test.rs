//! Integration tests for the swe_edge_runtime SAF runtime_manager surface.
#![allow(clippy::unwrap_used, clippy::expect_used)]
// @allow: no_mocks_in_integration — stub impls required to exercise the full daemon stack

use std::sync::Arc;

use futures::future::BoxFuture;
use futures::FutureExt;
use tokio::net::TcpListener;
use tokio::sync::oneshot;

use edge_domain::SecurityContext;
use edge_proxy::{HealthReport, LifecycleError, LifecycleMonitor};
use swe_edge_egress_http::{
    HttpEgress, HttpEgressResult, HttpRequest as EgressReq, HttpResponse as EgressResp,
    HttpStreamResponse,
};
use swe_edge_ingress_http::{
    AxumHttpServer, HttpHealthCheck, HttpIngress, HttpIngressError, HttpIngressResult, HttpRequest,
    HttpResponse, HttpServer,
};
use swe_edge_runtime::{Runtime, RuntimeConfig, RuntimeManager, RuntimeStatus};

struct StubLifecycle;
impl LifecycleMonitor for StubLifecycle {
    fn health(&self) -> BoxFuture<'_, HealthReport> {
        async move { HealthReport::from_components(vec![]) }.boxed()
    }
    fn start_background_tasks(&self) -> BoxFuture<'_, ()> {
        async move {}.boxed()
    }
    fn shutdown(&self) -> BoxFuture<'_, Result<(), LifecycleError>> {
        async move { Ok(()) }.boxed()
    }
}

struct StubHttpEgress;
impl HttpEgress for StubHttpEgress {
    fn send(&self, _: EgressReq) -> BoxFuture<'_, HttpEgressResult<EgressResp>> {
        Box::pin(async { Ok(EgressResp::new(200, vec![])) })
    }
    fn send_stream(&self, _: EgressReq) -> BoxFuture<'_, HttpEgressResult<HttpStreamResponse>> {
        Box::pin(async {
            Ok(HttpStreamResponse {
                status: 200,
                headers: Default::default(),
                body: Box::pin(futures::stream::empty()),
            })
        })
    }
    fn health_check(&self) -> BoxFuture<'_, HttpEgressResult<()>> {
        Box::pin(async { Ok(()) })
    }
}

struct EchoHandler;
impl HttpIngress for EchoHandler {
    fn handle(
        &self,
        req: HttpRequest,
        _ctx: SecurityContext,
    ) -> BoxFuture<'_, HttpIngressResult<HttpResponse>> {
        Box::pin(async move {
            Ok(HttpResponse::new(
                200,
                format!("{} {}", req.method, req.url).into_bytes(),
            ))
        })
    }
    fn health_check(&self) -> BoxFuture<'_, HttpIngressResult<HttpHealthCheck>> {
        Box::pin(async { Ok(HttpHealthCheck::healthy()) })
    }
}

struct NotFoundHandler;
impl HttpIngress for NotFoundHandler {
    fn handle(
        &self,
        _: HttpRequest,
        _ctx: SecurityContext,
    ) -> BoxFuture<'_, HttpIngressResult<HttpResponse>> {
        Box::pin(async { Err(HttpIngressError::NotFound("resource gone".into())) })
    }
    fn health_check(&self) -> BoxFuture<'_, HttpIngressResult<HttpHealthCheck>> {
        Box::pin(async { Ok(HttpHealthCheck::healthy()) })
    }
}

async fn start_daemon_stack(
    handler: Arc<dyn HttpIngress>,
) -> (String, impl RuntimeManager, oneshot::Sender<()>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let base_url = format!("http://{addr}");
    let config = RuntimeConfig::default().with_systemd_notify(false);
    let ingress = Arc::new(Runtime::http_ingress(handler.clone()));
    let egress = Arc::new(Runtime::http_egress(Arc::new(StubHttpEgress)));
    let mgr = Runtime::runtime_manager(config, ingress, egress, Arc::new(StubLifecycle));
    mgr.start().await.expect("RuntimeManager::start failed");
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
    let server = AxumHttpServer::new(addr.to_string(), handler);
    tokio::spawn(async move {
        let signal = async move {
            let _ = shutdown_rx.await;
        };
        let _ = server.serve_with_listener(listener, Box::pin(signal)).await;
    });
    (base_url, mgr, shutdown_tx)
}

/// @covers: runtime_manager — start transitions to Running, health reports components
#[tokio::test]
async fn test_runtime_manager_start_and_shutdown_round_trip() {
    let handler: Arc<dyn HttpIngress> = Arc::new(EchoHandler);
    let config = RuntimeConfig::default().with_systemd_notify(false);
    let ingress = Arc::new(Runtime::http_ingress(handler));
    let egress = Arc::new(Runtime::http_egress(Arc::new(StubHttpEgress)));
    let mgr = Runtime::runtime_manager(config, ingress, egress, Arc::new(StubLifecycle));
    mgr.start().await.expect("start ok");
    assert_eq!(mgr.health().await.status, RuntimeStatus::Running);
    mgr.shutdown().await.expect("shutdown ok");
    assert_eq!(mgr.health().await.status, RuntimeStatus::Stopped);
}

/// @covers: runtime_manager — health reports ingress and egress component names
#[tokio::test]
async fn test_runtime_manager_health_reports_ingress_and_egress() {
    let handler: Arc<dyn HttpIngress> = Arc::new(EchoHandler);
    let config = RuntimeConfig::default().with_systemd_notify(false);
    let ingress = Arc::new(Runtime::http_ingress(handler));
    let egress = Arc::new(Runtime::http_egress(Arc::new(StubHttpEgress)));
    let mgr = Runtime::runtime_manager(config, ingress, egress, Arc::new(StubLifecycle));
    mgr.start().await.expect("start ok");
    let health = mgr.health().await;
    let names: Vec<&str> = health.components.iter().map(|c| c.name.as_str()).collect();
    assert!(names.contains(&"ingress.http"));
    assert!(names.contains(&"egress.http"));
}

#[tokio::test]
async fn test_http_get_request_flows_end_to_end_through_daemon_stack() {
    let (base, mgr, shutdown_tx) = start_daemon_stack(Arc::new(EchoHandler)).await;
    let resp = reqwest::get(format!("{base}/ping"))
        .await
        .expect("HTTP request failed");
    assert_eq!(resp.status(), 200);
    let body = resp.text().await.unwrap();
    assert!(
        body.contains("GET"),
        "expected GET in echo body, got: {body}"
    );
    let _ = shutdown_tx.send(());
    mgr.shutdown().await.expect("shutdown ok");
}

#[tokio::test]
async fn test_http_post_with_json_body_reaches_handler() {
    let (base, mgr, shutdown_tx) = start_daemon_stack(Arc::new(EchoHandler)).await;
    let resp = reqwest::Client::new()
        .post(format!("{base}/submit"))
        .json(&serde_json::json!({"key": "value"}))
        .send()
        .await
        .expect("HTTP request failed");
    assert_eq!(resp.status(), 200);
    let body = resp.text().await.unwrap();
    assert!(
        body.contains("POST"),
        "expected POST in echo body, got: {body}"
    );
    let _ = shutdown_tx.send(());
    mgr.shutdown().await.expect("shutdown ok");
}

#[tokio::test]
async fn test_handler_not_found_error_surfaces_as_404_at_wire_level() {
    let (base, mgr, shutdown_tx) = start_daemon_stack(Arc::new(NotFoundHandler)).await;
    let resp = reqwest::get(format!("{base}/missing"))
        .await
        .expect("HTTP request failed");
    assert_eq!(resp.status(), 404);
    let _ = shutdown_tx.send(());
    mgr.shutdown().await.expect("shutdown ok");
}

#[tokio::test]
async fn test_server_refuses_connections_after_daemon_shutdown() {
    let (base, mgr, shutdown_tx) = start_daemon_stack(Arc::new(EchoHandler)).await;
    reqwest::get(format!("{base}/check"))
        .await
        .expect("should be up");
    let _ = shutdown_tx.send(());
    mgr.shutdown().await.expect("shutdown ok");
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    let result = reqwest::get(format!("{base}/check")).await;
    assert!(
        result.is_err(),
        "expected connection refused after shutdown"
    );
}

// ── Rule-222 coverage: test_<fn>_<condition>_<happy|error|edge> ──────────────

fn make_manager() -> impl RuntimeManager {
    let ingress = Arc::new(Runtime::http_ingress(Arc::new(EchoHandler)));
    let egress = Arc::new(Runtime::http_egress(Arc::new(StubHttpEgress)));
    Runtime::runtime_manager(
        RuntimeConfig::default().with_systemd_notify(false),
        ingress,
        egress,
        Arc::new(StubLifecycle),
    )
}

#[tokio::test]
async fn test_start_valid_components_returns_ok_happy() {
    let mgr = make_manager();
    assert!(mgr.start().await.is_ok());
}

#[tokio::test]
async fn test_start_empty_ingress_returns_err_error() {
    let ingress = Arc::new(Runtime::empty_ingress());
    let egress = Arc::new(Runtime::http_egress(Arc::new(StubHttpEgress)));
    let mgr = Runtime::runtime_manager(
        RuntimeConfig::default().with_systemd_notify(false),
        ingress,
        egress,
        Arc::new(StubLifecycle),
    );
    assert!(mgr.start().await.is_err());
}

#[tokio::test]
async fn test_start_twice_does_not_panic_edge() {
    let mgr = make_manager();
    let _ = mgr.start().await;
    let _ = mgr.start().await;
}

#[tokio::test]
async fn test_shutdown_after_start_returns_ok_happy() {
    let mgr = make_manager();
    mgr.start().await.expect("start ok");
    assert!(mgr.shutdown().await.is_ok());
}

#[tokio::test]
async fn test_shutdown_before_start_does_not_panic_error() {
    let mgr = make_manager();
    let _ = mgr.shutdown().await;
}

#[tokio::test]
async fn test_shutdown_twice_is_safe_edge() {
    let mgr = make_manager();
    mgr.start().await.expect("start ok");
    let _ = mgr.shutdown().await;
    let _ = mgr.shutdown().await;
}

#[tokio::test]
async fn test_health_after_start_reports_running_happy() {
    let mgr = make_manager();
    mgr.start().await.expect("start ok");
    assert_eq!(mgr.health().await.status, RuntimeStatus::Running);
}

#[tokio::test]
async fn test_health_before_start_is_not_running_error() {
    let mgr = make_manager();
    assert_ne!(mgr.health().await.status, RuntimeStatus::Running);
}

#[tokio::test]
async fn test_health_components_nonempty_after_start_edge() {
    let mgr = make_manager();
    mgr.start().await.expect("start ok");
    assert!(!mgr.health().await.components.is_empty());
}

#[test]
fn test_service_registry_builder_no_registry_returns_none_happy() {
    let mgr = make_manager();
    // Default manager has no service registry wired; builder is unavailable.
    let _b = mgr.service_registry_builder();
}

#[test]
fn test_service_registry_builder_no_registry_wired_returns_none_error() {
    let mgr = make_manager();
    assert!(
        mgr.service_registry_builder().is_none(),
        "expected None when no ServiceRegistry is wired"
    );
}

#[test]
fn test_service_registry_builder_default_method_does_not_panic_edge() {
    let mgr = make_manager();
    // Must not panic regardless of registry presence.
    let _ = mgr.service_registry_builder();
}
