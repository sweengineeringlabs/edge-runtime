//! [`crate::api::CliCommand`] impl for [`crate::api::NoopCliCommand`].

use crate::api::{CliArgs, CliCommand, NoopCliCommand};

impl CliCommand for NoopCliCommand {
    fn name(&self) -> &str {
        &self.name
    }

    fn args(&self) -> CliArgs {
        CliArgs {
            positional: self.positional.clone(),
            flags: self.flags.clone(),
        }
    }
}
