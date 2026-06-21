//! [`Validator`] — validates CLI arguments before command dispatch.

use crate::api::CliError;

/// Validates CLI arguments or command configuration.
///
/// Implement this trait to enforce pre-dispatch invariants on [`crate::api::CliArgs`]
/// before a [`crate::api::CliRunner`] executes a command.
pub trait Validator {
    /// Validate the target. Returns `Ok(())` when valid.
    fn validate(&self) -> Result<(), CliError>;
}
