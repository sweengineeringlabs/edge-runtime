//! SAF surface for [`crate::api::CliCommand`] and [`crate::api::NoopCliCommand`].

use std::collections::HashMap;

pub(crate) use crate::api::{CliArgs, NoopCliCommand};

impl NoopCliCommand {
    /// Create a [`NoopCliCommand`] with the given name and empty args.
    pub fn create(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            positional: Vec::new(),
            flags: HashMap::new(),
        }
    }

    /// Create a [`NoopCliCommand`] with the given name and args.
    pub fn create_with_args(name: impl Into<String>, args: CliArgs) -> Self {
        Self {
            name: name.into(),
            positional: args.positional,
            flags: args.flags,
        }
    }
}
