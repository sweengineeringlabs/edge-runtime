//! Integration tests for the [`CliRunner`] trait contract via [`NoopCliRunner`].
// @covers NoopCliRunner::run
#![allow(clippy::unwrap_used)]

use futures::executor::block_on;
use swe_edge_runtime_cli::{CliRunner, NoopCliCommand, NoopCliRunner};

// ─── run ────────────────────────────────────────────────────────────────────

#[test]
fn test_run_returns_success_for_known_command_happy() {
    let runner = NoopCliRunner::create();
    let cmd = NoopCliCommand::create("list");
    let out = block_on(runner.run(&cmd)).unwrap();
    assert!(out.is_success());
    assert_eq!(out.exit_code, 0);
}

#[test]
fn test_run_noop_does_not_error_for_unknown_command_error() {
    let runner = NoopCliRunner::create();
    let cmd = NoopCliCommand::create("nonexistent");
    let out = block_on(runner.run(&cmd)).unwrap();
    assert_eq!(
        out.exit_code, 0,
        "Noop must not error regardless of command name"
    );
}

#[test]
fn test_run_callable_through_dyn_trait_object_edge() {
    let runner: &dyn CliRunner = &NoopCliRunner::create();
    let cmd = NoopCliCommand::create("inspect");
    let out = block_on(runner.run(&cmd)).unwrap();
    assert!(out.is_success());
}
