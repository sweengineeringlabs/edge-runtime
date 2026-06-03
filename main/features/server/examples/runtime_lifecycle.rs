//! Runtime lifecycle — config loading, gateway assembly, start, health, shutdown.
//!
//! Run:
//!     cargo run -p swe-edge-runtime --example runtime_lifecycle
//!
//! Demonstrates:
//!   1. RuntimeConfig loading and customisation via the builder API
//!   2. DefaultInput + DefaultOutput assembly from SAF constructors
//!   3. RuntimeManager lifecycle: start → health → shutdown
//!
//! SEA constraint: public API is accessed only through each crate's SAF surface.
//! The stub inbound/outbound adapters are defined locally — they stand in for
//! the real axum/tonic/reqwest implementations a consumer app would supply.

use std::sync::Arc;

use edge_proxy::new_null_lifecycle_monitor;
use futures::future::BoxFuture;
use swe_edge_egress_http::{
    HttpEgress, HttpEgressResult, HttpRequest as EgressReq, HttpResponse as EgressResp,
    HttpStreamResponse,
};
use swe_edge_ingress_http::{
    HttpHealthCheck, HttpIngress, HttpIngressResult, HttpRequest, HttpResponse, RequestContext,
};
use swe_edge_runtime::{Runtime, RuntimeConfig, RuntimeManager, RuntimeStatus};

// ── stub inbound ──────────────────────────────────────────────────────────────

struct NoopIngress;

impl HttpIngress for NoopIngress {
    fn handle(
        &self,
        _: HttpRequest,
        _ctx: RequestContext,
    ) -> BoxFuture<'_, HttpIngressResult<HttpResponse>> {
        Box::pin(async { Ok(HttpResponse::new(204, vec![])) })
    }

    fn health_check(&self) -> BoxFuture<'_, HttpIngressResult<HttpHealthCheck>> {
        Box::pin(async { Ok(HttpHealthCheck::healthy()) })
    }
}

// ── stub outbound ─────────────────────────────────────────────────────────────

struct NoopEgress;

impl HttpEgress for NoopEgress {
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

// ── main ──────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Config — customise the defaults via the builder API.
    let config = RuntimeConfig::default()
        .with_service_name("example-service")
        .with_http_bind("127.0.0.1:8080")
        .with_shutdown_timeout(5);
    println!("service:  {}", config.service_name);
    println!("http:     {}", config.http_bind);
    println!("timeout:  {}s", config.shutdown_timeout_secs);

    // 2. Assemble gateways — Runtime::http_ingress/http_egress accept `Arc<dyn Trait>`
    //    so the runtime never names or imports concrete adapter types.
    let ingress = Arc::new(Runtime::http_ingress(Arc::new(NoopIngress)));
    let egress = Arc::new(Runtime::http_egress(Arc::new(NoopEgress)));
    let lifecycle = new_null_lifecycle_monitor();

    // 3. Build the RuntimeManager via the SAF factory (returns `impl RuntimeManager`).
    let mgr = Runtime::runtime_manager(config, ingress, egress, lifecycle);

    // 4. Start — probes each configured transport and transitions status to Running.
    mgr.start().await?;
    let health = mgr.health().await;
    println!("\nafter start:");
    println!("  status: {:?}", health.status);
    assert_eq!(health.status, RuntimeStatus::Running);
    for c in &health.components {
        println!("  component {} — healthy={}", c.name, c.healthy);
    }

    // 5. Shutdown — drains in-flight work and transitions status to Stopped.
    mgr.shutdown().await?;
    let health = mgr.health().await;
    println!("\nafter shutdown:");
    println!("  status: {:?}", health.status);
    assert_eq!(health.status, RuntimeStatus::Stopped);

    Ok(())
}
