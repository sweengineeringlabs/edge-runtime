//! No-op [`CliRunner`] implementation.

use futures::future::BoxFuture;

use crate::api::{CliArgs, CliError, CliOutput, CliRunner, NoopCliRunner};

impl CliRunner for NoopCliRunner {
    fn run(&self, _name: &str, _args: &CliArgs) -> BoxFuture<'_, Result<CliOutput, CliError>> {
        Box::pin(async { Ok(CliOutput::success("")) })
    }
}
