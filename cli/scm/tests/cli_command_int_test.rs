//! Integration tests for the [`CliCommand`] trait contract via [`NoopCliCommand`].
// @covers NoopCliCommand::name
// @covers NoopCliCommand::args
#![allow(clippy::unwrap_used)]

use swe_edge_runtime_cli::{CliArgs, CliCommand, NoopCliCommand};

// ─── name ────────────────────────────────────────────────────────────────────

#[test]
fn test_name_returns_provided_value_happy() {
    let cmd = NoopCliCommand::create("deploy");
    assert_eq!(cmd.name(), "deploy");
}

#[test]
fn test_name_with_special_chars_is_preserved_error() {
    let cmd = NoopCliCommand::create("my-command:v2");
    assert_eq!(cmd.name(), "my-command:v2");
}

#[test]
fn test_name_callable_through_dyn_trait_object_edge() {
    let cmd = NoopCliCommand::create("inspect");
    let dyn_cmd: &dyn CliCommand = &cmd;
    assert_eq!(dyn_cmd.name(), "inspect");
}

// ─── args ────────────────────────────────────────────────────────────────────

#[test]
fn test_args_returns_empty_by_default_happy() {
    let cmd = NoopCliCommand::create("list");
    assert!(cmd.args().is_empty());
}

#[test]
fn test_args_returns_provided_args_error() {
    let mut args = CliArgs::new();
    args.positional.push("target".into());
    let cmd = NoopCliCommand::create_with_args("run", args);
    assert_eq!(cmd.args().get(0), Some("target"));
}

#[test]
fn test_args_callable_through_dyn_trait_object_edge() {
    let cmd = NoopCliCommand::create("inspect");
    let dyn_cmd: &dyn CliCommand = &cmd;
    assert!(dyn_cmd.args().is_empty());
}
