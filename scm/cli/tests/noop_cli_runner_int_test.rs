//! Integration tests for [`NoopCliRunner`].
// @covers NoopCliRunner::run
#![allow(clippy::unwrap_used)]

use futures::executor::block_on;
use swe_edge_runtime_cli::{CliArgs, CliRunner, NoopCliRunner};

/// @covers: NoopCliRunner::run
#[test]
fn test_run_returns_success_output_happy() {
    let runner = NoopCliRunner::create();
    let out = block_on(runner.run("list", &CliArgs::new())).unwrap();
    assert!(out.is_success());
}

/// @covers: NoopCliRunner::run
#[test]
fn test_run_ignores_command_name_and_succeeds_error() {
    let runner = NoopCliRunner::create();
    let out = block_on(runner.run("nonexistent-command", &CliArgs::new())).unwrap();
    assert!(
        out.is_success(),
        "noop must succeed regardless of command name"
    );
}

/// @covers: NoopCliRunner::run
#[test]
fn test_run_called_twice_is_independent_edge() {
    let runner = NoopCliRunner::create();
    let a = block_on(runner.run("a", &CliArgs::new())).unwrap();
    let b = block_on(runner.run("b", &CliArgs::new())).unwrap();
    assert!(a.is_success());
    assert!(b.is_success());
}
