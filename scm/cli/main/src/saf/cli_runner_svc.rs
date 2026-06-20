//! SAF surface for the CLI runner contract.

pub use crate::api::{CliArgs, CliError, CliOutput, CliRunner, NoopCliRunner};

impl NoopCliRunner {
    /// Create a no-op runner that always succeeds.
    pub fn create() -> Self {
        Self
    }
}
