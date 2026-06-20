//! Demonstrates the handler-composition pattern (ADR-047 Phase 4):
//! HTTP POST → GreetHandler (HttpIngress) → ServiceRegistry → CliRunner → CliOutput → HTTP 200.
//!
//! Shows:
//!   1. An `HttpIngress` handler holding `Arc<ServiceRegistry>`
//!   2. JSON request body decoded into `GreetRequest`
//!   3. Dispatch to the wired `CliRunner` via `registry.cli_runner()`
//!   4. CLI output mapped to a JSON `GreetResponse`
//!   5. Self-contained round-trip: bind on :0, POST /greet, print result, shut down

use std::sync::Arc;

use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::sync::oneshot;

use edge_domain::SecurityContext;
use swe_edge_egress_http::HttpTransportSvc;
use swe_edge_ingress_http::{
    AxumHttpServer, HttpBody, HttpHealthCheck, HttpIngress, HttpIngressError, HttpIngressResult,
    HttpRequest, HttpResponse, HttpServer,
};
use swe_edge_runtime::Runtime;
use swe_edge_runtime_cli::{CliArgs, CliCommand, CliError, CliOutput, CliRunner};

// ── Domain request / response ─────────────────────────────────────────────────

#[derive(Deserialize)]
struct GreetRequest {
    name: String,
}

#[derive(Deserialize, Serialize)]
struct GreetResponse {
    message: String,
}

// ── GreetCommand ──────────────────────────────────────────────────────────────

struct GreetCommand {
    name: String,
}

impl CliCommand for GreetCommand {
    fn name(&self) -> &str {
        "greet"
    }

    fn args(&self) -> CliArgs {
        let mut args = CliArgs::new();
        args.positional.push(self.name.clone());
        args
    }
}

// ── GreetRunner ───────────────────────────────────────────────────────────────

struct GreetRunner;

impl CliRunner for GreetRunner {
    fn run(&self, command: &dyn CliCommand) -> BoxFuture<'_, Result<CliOutput, CliError>> {
        let cmd_name = command.name().to_owned();
        let subject = command.args().get(0).unwrap_or("world").to_owned();
        Box::pin(async move {
            match cmd_name.as_str() {
                "greet" => Ok(CliOutput::success(format!("Hello, {subject}!"))),
                other => Err(CliError::CommandNotFound(other.to_owned())),
            }
        })
    }
}

// ── GreetHandler (the composition layer) ─────────────────────────────────────

struct GreetHandler {
    registry: Arc<swe_edge_runtime::ServiceRegistry>,
}

impl HttpIngress for GreetHandler {
    fn handle(
        &self,
        req: HttpRequest,
        _ctx: SecurityContext,
    ) -> BoxFuture<'_, HttpIngressResult<HttpResponse>> {
        let registry = Arc::clone(&self.registry);
        Box::pin(async move {
            // AxumHttpServer parses application/json bodies into HttpBody::Json.
            // Fall back to raw bytes so the handler works with any JSON-producing client.
            let val = match req.body {
                Some(HttpBody::Json(v)) => v,
                Some(HttpBody::Raw(bytes)) => serde_json::from_slice(&bytes)
                    .map_err(|e| HttpIngressError::InvalidInput(e.to_string()))?,
                _ => {
                    return Err(HttpIngressError::InvalidInput(
                        r#"expected JSON body {"name":"..."}"#.into(),
                    ));
                }
            };

            let greq: GreetRequest = serde_json::from_value(val)
                .map_err(|e| HttpIngressError::InvalidInput(e.to_string()))?;

            let runner = registry.cli_runner().ok_or_else(|| {
                HttpIngressError::Internal("no CliRunner wired into ServiceRegistry".into())
            })?;

            let output = runner
                .run(&GreetCommand { name: greq.name })
                .await
                .map_err(|e| HttpIngressError::Internal(e.to_string()))?;

            let body = serde_json::to_vec(&GreetResponse {
                message: output.stdout,
            })
            .map_err(|e| HttpIngressError::Internal(e.to_string()))?;

            Ok(HttpResponse::new(200, body))
        })
    }

    fn health_check(&self) -> BoxFuture<'_, HttpIngressResult<HttpHealthCheck>> {
        Box::pin(async { Ok(HttpHealthCheck::healthy()) })
    }
}

// ── Entry point ───────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    let http = Arc::from(
        HttpTransportSvc::default_http_egress()
            .unwrap_or_else(|e| panic!("http egress init failed: {e}")),
    );

    let registry = Runtime::builder()
        .egress_http(http)
        .with_cli_runner(GreetRunner)
        .build_registry()
        .unwrap_or_else(|| panic!("build_registry returned None — egress_http is required"));

    let handler: Arc<dyn HttpIngress> = Arc::new(GreetHandler {
        registry: Arc::clone(&registry),
    });

    // Bind on an ephemeral port so the example is self-contained and port-collision-free.
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .unwrap_or_else(|e| panic!("bind failed: {e}"));
    let addr = listener
        .local_addr()
        .unwrap_or_else(|e| panic!("local_addr: {e}"));
    let base = format!("http://{addr}");

    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
    let server = AxumHttpServer::new(addr.to_string(), handler);
    tokio::spawn(async move {
        let signal = async move {
            let _ = shutdown_rx.await;
        };
        let _ = server.serve_with_listener(listener, Box::pin(signal)).await;
    });

    // Brief pause so the TCP listener is ready to accept before the client connects.
    tokio::time::sleep(std::time::Duration::from_millis(20)).await;

    // ── Happy path: greet Alice ───────────────────────────────────────────────
    let resp = reqwest::Client::new()
        .post(format!("{base}/greet"))
        .json(&serde_json::json!({"name": "Alice"}))
        .send()
        .await
        .unwrap_or_else(|e| panic!("POST /greet failed: {e}"));

    let status = resp.status();
    let body: GreetResponse = resp
        .json()
        .await
        .unwrap_or_else(|e| panic!("decode GreetResponse: {e}"));

    println!("POST /greet → status={status} message={:?}", body.message);

    // ── Shutdown ──────────────────────────────────────────────────────────────
    let _ = shutdown_tx.send(());
    println!("server shut down — example complete");
}
