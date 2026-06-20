//! CLI runner contract — executes a parsed [`super::CliCommand`].

use futures::future::BoxFuture;

use super::cli_command::CliCommand;
use crate::api::{CliError, CliOutput};

/// Executes a parsed [`CliCommand`] and returns a [`CliOutput`].
///
/// Implement this trait in a plugin or composition root to provide the
/// concrete command-dispatch strategy.
pub trait CliRunner: Send + Sync {
    /// Execute the given parsed command.
    fn run(&self, command: &dyn CliCommand) -> BoxFuture<'_, Result<CliOutput, CliError>>;
}
