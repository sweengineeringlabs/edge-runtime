//! Integration tests for the `cli_command_svc` SAF surface.
// @covers NoopCliCommand::create
// @covers NoopCliCommand::create_with_args
#![allow(clippy::unwrap_used)]

use swe_edge_runtime_cli::{CliArgs, CliCommand, NoopCliCommand};

// ─── create ──────────────────────────────────────────────────────────────────

/// @covers: NoopCliCommand::create
#[test]
fn test_create_returns_command_with_correct_name_happy() {
    let cmd = NoopCliCommand::create("deploy");
    assert_eq!(cmd.name(), "deploy");
    assert!(cmd.args().is_empty());
}

/// @covers: NoopCliCommand::create
#[test]
fn test_create_with_unusual_name_does_not_error_error() {
    let cmd = NoopCliCommand::create("!!edge-case!!");
    assert_eq!(cmd.name(), "!!edge-case!!");
}

/// @covers: NoopCliCommand::create
#[test]
fn test_create_independent_instances_have_distinct_names_edge() {
    let a = NoopCliCommand::create("alpha");
    let b = NoopCliCommand::create("beta");
    assert_ne!(a.name(), b.name());
}

// ─── create_with_args ────────────────────────────────────────────────────────

/// @covers: NoopCliCommand::create_with_args
#[test]
fn test_create_with_args_exposes_flag_through_trait_happy() {
    let mut args = CliArgs::new();
    args.flags.insert("format".into(), "json".into());
    let cmd = NoopCliCommand::create_with_args("list", args);
    let dyn_cmd: &dyn CliCommand = &cmd;
    assert_eq!(dyn_cmd.args().flag("format"), Some("json"));
}

/// @covers: NoopCliCommand::create_with_args
#[test]
fn test_create_with_args_empty_args_is_valid_error() {
    let cmd = NoopCliCommand::create_with_args("run", CliArgs::new());
    assert!(cmd.args().is_empty());
}

/// @covers: NoopCliCommand::create_with_args
#[test]
fn test_create_with_args_positional_preserved_edge() {
    let mut args = CliArgs::new();
    args.positional.extend(["a".into(), "b".into()]);
    let cmd = NoopCliCommand::create_with_args("exec", args);
    assert_eq!(cmd.args().get(0), Some("a"));
    assert_eq!(cmd.args().get(1), Some("b"));
}
