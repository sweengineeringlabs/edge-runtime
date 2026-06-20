//! CLI runner contract — executes a named command with its arguments.

use futures::future::BoxFuture;

use crate::api::{CliArgs, CliError, CliOutput};

/// Executes a named CLI command and returns a [`CliOutput`].
///
/// Implement this trait in a plugin or composition root to provide the
/// concrete command-dispatch strategy.
pub trait CliRunner: Send + Sync {
    /// Execute the command identified by `name` with the given `args`.
    fn run(&self, name: &str, args: &CliArgs) -> BoxFuture<'_, Result<CliOutput, CliError>>;
}
