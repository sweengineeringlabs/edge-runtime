//! Integration tests for [`CliArgs`].
#![allow(clippy::unwrap_used)]

use swe_edge_runtime_cli::CliArgs;

/// @covers: CliArgs::new
#[test]
fn test_new_creates_empty_args_happy() {
    let args = CliArgs::new();
    assert!(args.is_empty());
}

/// @covers: CliArgs::get
#[test]
fn test_get_returns_none_for_missing_index_error() {
    let args = CliArgs::new();
    assert!(args.get(0).is_none());
}

/// @covers: CliArgs::get, CliArgs::flag
#[test]
fn test_get_and_flag_on_populated_args_edge() {
    let mut args = CliArgs::new();
    args.positional.push("run".to_string());
    args.flags.insert("output".to_string(), "json".to_string());
    assert_eq!(args.get(0), Some("run"));
    assert_eq!(args.flag("output"), Some("json"));
    assert!(args.get(1).is_none());
    assert!(args.flag("missing").is_none());
}
