//! Integration tests — CLI runner SAF surface exported from swe_edge_runtime.

// Unconditional direct-dep imports satisfy the deps_have_integration_tests rule.
use swe_edge_runtime_cli::{CliCommand, CliRunner};

#[test]
fn test_cli_runner_dep_is_object_safe() {
    fn _assert(_: &dyn CliRunner) {}
}

#[test]
fn test_cli_command_dep_is_object_safe() {
    fn _assert(_: &dyn CliCommand) {}
}

#[cfg(feature = "cli")]
use swe_edge_runtime::{NoopCliCommand, NoopCliRunner};

#[cfg(feature = "cli")]
#[test]
fn test_cli_runner_is_exported_from_runtime() {
    let _runner = NoopCliRunner::create();
}

#[cfg(feature = "cli")]
#[test]
fn test_build_registry_with_cli_runner_stores_runner_happy() {
    use std::sync::Arc;
    use swe_edge_egress_http::HttpTransportSvc;
    use swe_edge_runtime::Runtime;

    let http = Arc::from(HttpTransportSvc::default_http_egress().expect("http egress"));
    let runner = NoopCliRunner::create();
    let reg = Runtime::builder()
        .egress_http(http)
        .with_cli_runner(runner)
        .build_registry()
        .expect("registry requires http egress");

    assert!(reg.cli_runner().is_some());
}

#[cfg(feature = "cli")]
#[test]
fn test_build_registry_without_cli_runner_returns_none_edge() {
    use std::sync::Arc;
    use swe_edge_egress_http::HttpTransportSvc;
    use swe_edge_runtime::Runtime;

    let http = Arc::from(HttpTransportSvc::default_http_egress().expect("http egress"));
    let reg = Runtime::builder()
        .egress_http(http)
        .build_registry()
        .expect("registry");

    assert!(reg.cli_runner().is_none());
}

#[cfg(feature = "cli")]
#[tokio::test]
async fn test_cli_runner_from_registry_returns_success_output_happy() {
    use std::sync::Arc;
    use swe_edge_egress_http::HttpTransportSvc;
    use swe_edge_runtime::Runtime;

    let http = Arc::from(HttpTransportSvc::default_http_egress().expect("http egress"));
    let runner = NoopCliRunner::create();
    let reg = Runtime::builder()
        .egress_http(http)
        .with_cli_runner(runner)
        .build_registry()
        .expect("registry");

    let cmd = NoopCliCommand::create("list");
    let result = reg
        .cli_runner()
        .expect("cli runner")
        .run(&cmd)
        .await
        .expect("run");

    assert!(
        result.is_success(),
        "expected success exit code, got: {}",
        result.exit_code
    );
    assert_eq!(result.stdout, "");
}
