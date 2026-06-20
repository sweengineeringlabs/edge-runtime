//! [`CliCommand`] — a parsed, dispatchable command passed to [`super::CliRunner`].

use crate::api::CliArgs;

/// A parsed CLI command ready for dispatch.
///
/// Implement this trait to represent a concrete parsed command (e.g. `RunCommand`,
/// `DeployCommand`). [`crate::NoopCliCommand`] is provided for tests.
pub trait CliCommand {
    /// The name of the command (e.g. `"run"`, `"deploy"`).
    fn name(&self) -> &str;
    /// The parsed arguments for this command.
    fn args(&self) -> CliArgs;
}
