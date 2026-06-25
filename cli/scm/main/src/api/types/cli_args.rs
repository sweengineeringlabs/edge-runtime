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
