//! Demonstrates wiring a custom [`CliRunner`] into [`RuntimeBuilder`] (ADR-047 Phase 4).
//!
//! Shows:
//!   1. A concrete `CliCommand` implementation (`HelloCommand`)
//!   2. A concrete `CliRunner` implementation (`HelloRunner`) that dispatches by name
//!   3. Wiring into `Runtime::builder().with_cli_runner()`
//!   4. Accessing the runner from `ServiceRegistry::cli_runner()`
//!   5. Success path (`greet`) and error path (`CommandNotFound`)

use std::sync::Arc;

use futures::future::BoxFuture;
use swe_edge_egress_http::HttpTransportSvc;
use swe_edge_runtime::Runtime;
use swe_edge_runtime_cli::{CliArgs, CliCommand, CliError, CliOutput, CliRunner};

// ── HelloCommand ─────────────────────────────────────────────────────────────

struct HelloCommand {
    subject: String,
}

impl HelloCommand {
    fn new(subject: impl Into<String>) -> Self {
        Self {
            subject: subject.into(),
        }
    }
}

impl CliCommand for HelloCommand {
    fn name(&self) -> &str {
        "greet"
    }

    fn args(&self) -> CliArgs {
        let mut args = CliArgs::new();
        args.positional.push(self.subject.clone());
        args
    }
}

// ── UnknownCommand (error-path fixture) ──────────────────────────────────────

struct UnknownCommand {
    name: String,
}

impl CliCommand for UnknownCommand {
    fn name(&self) -> &str {
        &self.name
    }

    fn args(&self) -> CliArgs {
        CliArgs::new()
    }
}

// ── HelloRunner ──────────────────────────────────────────────────────────────

struct HelloRunner;

impl CliRunner for HelloRunner {
    fn run(&self, command: &dyn CliCommand) -> BoxFuture<'_, Result<CliOutput, CliError>> {
        let name = command.name().to_owned();
        let subject = command.args().get(0).unwrap_or("world").to_owned();
        Box::pin(async move {
            match name.as_str() {
                "greet" => Ok(CliOutput::success(format!("Hello, {subject}!"))),
                other => Err(CliError::CommandNotFound(other.to_owned())),
            }
        })
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
        .with_cli_runner(HelloRunner)
        .build_registry()
        .unwrap_or_else(|| panic!("build_registry returned None — egress_http is required"));

    let runner = registry
        .cli_runner()
        .unwrap_or_else(|| panic!("no cli runner wired into registry"));

    // ── Success path ──────────────────────────────────────────────────────────
    let out = runner
        .run(&HelloCommand::new("world"))
        .await
        .unwrap_or_else(|e| panic!("greet command failed: {e}"));
    println!(
        "greet → exit_code={} stdout={:?}",
        out.exit_code, out.stdout
    );

    // ── Error path ────────────────────────────────────────────────────────────
    let unknown = UnknownCommand {
        name: "deploy".to_owned(),
    };
    match runner.run(&unknown).await {
        Err(CliError::CommandNotFound(cmd)) => {
            println!("deploy → CommandNotFound({cmd:?}) — expected");
        }
        Ok(out) => eprintln!("deploy → unexpected success: {:?}", out.stdout),
        Err(e) => eprintln!("deploy → unexpected error variant: {e}"),
    }
}
