//! Parsed CLI argument bag.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Parsed CLI arguments: positional arguments and named flags.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CliArgs {
    /// Positional arguments in order.
    pub positional: Vec<String>,
    /// Named flags (e.g. `--output json` → `"output" => "json"`).
    pub flags: HashMap<String, String>,
}

impl CliArgs {
    /// Construct an empty argument bag.
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the positional argument at `index`, if present.
    pub fn get(&self, index: usize) -> Option<&str> {
        self.positional.get(index).map(String::as_str)
    }

    /// Return the value of a named flag, if present.
    pub fn flag(&self, name: &str) -> Option<&str> {
        self.flags.get(name).map(String::as_str)
    }

    /// Returns `true` if there are no positional args and no flags.
    pub fn is_empty(&self) -> bool {
        self.positional.is_empty() && self.flags.is_empty()
    }
}
