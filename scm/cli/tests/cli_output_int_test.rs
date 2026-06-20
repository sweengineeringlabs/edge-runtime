//! Integration tests for [`CliOutput`].

use swe_edge_runtime_cli::CliOutput;

/// @covers: CliOutput::success
#[test]
fn test_success_sets_exit_code_zero_happy() {
    let out = CliOutput::success("hello");
    assert!(out.is_success());
    assert_eq!(out.stdout, "hello");
    assert_eq!(out.exit_code, 0);
}

/// @covers: CliOutput::is_success
#[test]
fn test_is_success_false_for_nonzero_exit_code_error() {
    let out = CliOutput::new("", "boom", 1);
    assert!(!out.is_success());
    assert_eq!(out.stderr, "boom");
}

/// @covers: CliOutput::new
#[test]
fn test_new_preserves_all_fields_edge() {
    let out = CliOutput::new("out", "err", 42);
    assert_eq!(out.stdout, "out");
    assert_eq!(out.stderr, "err");
    assert_eq!(out.exit_code, 42);
}
