//! Integration tests for [`NoopCliRunner`].
// @covers NoopCliRunner::run
#![allow(clippy::unwrap_used)]

use futures::executor::block_on;
use swe_edge_runtime_cli::{CliRunner, NoopCliCommand, NoopCliRunner};

/// @covers: NoopCliRunner::run
#[test]
fn test_run_returns_success_output_happy() {
    let runner = NoopCliRunner::create();
    let cmd = NoopCliCommand::create("list");
    let out = block_on(runner.run(&cmd)).unwrap();
    assert!(out.is_success());
}

/// @covers: NoopCliRunner::run
#[test]
fn test_run_ignores_command_name_and_succeeds_error() {
    let runner = NoopCliRunner::create();
    let cmd = NoopCliCommand::create("nonexistent-command");
    let out = block_on(runner.run(&cmd)).unwrap();
    assert!(
        out.is_success(),
        "noop must succeed regardless of command name"
    );
}

/// @covers: NoopCliRunner::run
#[test]
fn test_run_called_twice_is_independent_edge() {
    let runner = NoopCliRunner::create();
    let a = block_on(runner.run(&NoopCliCommand::create("a"))).unwrap();
    let b = block_on(runner.run(&NoopCliCommand::create("b"))).unwrap();
    assert!(a.is_success());
    assert!(b.is_success());
}
