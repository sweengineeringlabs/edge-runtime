//! Integration tests for [`NoopCliCommand`].
// @covers NoopCliCommand::create
// @covers NoopCliCommand::create_with_args
#![allow(clippy::unwrap_used)]

use swe_edge_runtime_cli::{CliArgs, CliCommand, NoopCliCommand};

// ─── create ──────────────────────────────────────────────────────────────────

#[test]
fn test_create_stores_name_happy() {
    let cmd = NoopCliCommand::create("run");
    assert_eq!(cmd.name(), "run");
}

#[test]
fn test_create_empty_name_is_valid_error() {
    let cmd = NoopCliCommand::create("");
    assert_eq!(cmd.name(), "");
}

#[test]
fn test_create_two_instances_are_independent_edge() {
    let a = NoopCliCommand::create("a");
    let b = NoopCliCommand::create("b");
    assert_ne!(a.name(), b.name());
}

// ─── create_with_args ────────────────────────────────────────────────────────

#[test]
fn test_create_with_args_stores_positional_happy() {
    let mut args = CliArgs::new();
    args.positional.push("file.txt".into());
    let cmd = NoopCliCommand::create_with_args("upload", args);
    assert_eq!(cmd.args().get(0), Some("file.txt"));
}

#[test]
fn test_create_with_args_stores_flags_error() {
    let mut args = CliArgs::new();
    args.flags.insert("output".into(), "json".into());
    let cmd = NoopCliCommand::create_with_args("list", args);
    assert_eq!(cmd.args().flag("output"), Some("json"));
}

#[test]
fn test_create_with_args_empty_args_is_valid_edge() {
    let cmd = NoopCliCommand::create_with_args("run", CliArgs::new());
    assert!(cmd.args().is_empty());
}
