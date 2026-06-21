//! Integration tests for [`CliError`].

use swe_edge_runtime_cli::CliError;

/// @covers: CliError::CommandNotFound
#[test]
fn test_command_not_found_formats_message_happy() {
    let e = CliError::CommandNotFound("foo".to_string());
    assert!(e.to_string().contains("foo"));
}

/// @covers: CliError::InvalidArgs
#[test]
fn test_invalid_args_formats_message_error() {
    let e = CliError::InvalidArgs("missing --output".to_string());
    assert!(e.to_string().contains("missing --output"));
}

/// @covers: CliError::ExecutionFailed, CliError::Io
#[test]
fn test_all_variants_produce_nonempty_messages_edge() {
    let variants: &[CliError] = &[
        CliError::CommandNotFound("x".into()),
        CliError::InvalidArgs("x".into()),
        CliError::ExecutionFailed("x".into()),
        CliError::Io("x".into()),
    ];
    for v in variants {
        assert!(!v.to_string().is_empty());
    }
}
