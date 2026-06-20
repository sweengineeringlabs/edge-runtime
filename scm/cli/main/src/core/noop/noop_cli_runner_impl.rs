//! No-op [`CliRunner`] implementation.

use futures::future::BoxFuture;

use crate::api::{CliCommand, CliError, CliOutput, CliRunner, NoopCliRunner};

impl CliRunner for NoopCliRunner {
    fn run(&self, _command: &dyn CliCommand) -> BoxFuture<'_, Result<CliOutput, CliError>> {
        Box::pin(async { Ok(CliOutput::success("")) })
    }
}
