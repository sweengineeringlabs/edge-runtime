//! Integration tests for the `cli_runner_svc` SAF surface.
// @covers NoopCliRunner::create
#![allow(clippy::unwrap_used)]

use futures::executor::block_on;
use swe_edge_runtime_cli::{CliArgs, CliRunner, NoopCliRunner};

/// @covers: NoopCliRunner::create
#[test]
fn test_create_returns_usable_runner_happy() {
    let runner = NoopCliRunner::create();
    let out = block_on(runner.run("ping", &CliArgs::new())).unwrap();
    assert!(out.is_success());
}

/// @covers: NoopCliRunner::create
#[test]
fn test_create_is_zero_sized_error() {
    assert_eq!(std::mem::size_of::<NoopCliRunner>(), 0);
}

/// @covers: NoopCliRunner::create
#[test]
fn test_create_callable_as_dyn_cli_runner_edge() {
    let runner: &dyn CliRunner = &NoopCliRunner::create();
    let out = block_on(runner.run("dyn", &CliArgs::new())).unwrap();
    assert!(out.is_success());
}
