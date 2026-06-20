//! Integration tests for the [`CliRunner`] trait contract via [`NoopCliRunner`].
// @covers NoopCliRunner::run
#![allow(clippy::unwrap_used)]

use futures::executor::block_on;
use swe_edge_runtime_cli::{CliArgs, CliRunner, NoopCliRunner};

// ─── run ────────────────────────────────────────────────────────────────────

#[test]
fn test_run_returns_success_for_known_command_happy() {
    let runner = NoopCliRunner::create();
    let out = block_on(runner.run("list", &CliArgs::new())).unwrap();
    assert!(out.is_success());
    assert_eq!(out.exit_code, 0);
}

#[test]
fn test_run_noop_does_not_error_for_unknown_command_error() {
    let runner = NoopCliRunner::create();
    let result = block_on(runner.run("nonexistent", &CliArgs::new()));
    assert!(
        result.is_ok(),
        "Noop must not error regardless of command name"
    );
}

#[test]
fn test_run_callable_through_dyn_trait_object_edge() {
    let runner: &dyn CliRunner = &NoopCliRunner::create();
    let out = block_on(runner.run("inspect", &CliArgs::new())).unwrap();
    assert!(out.is_success());
}
