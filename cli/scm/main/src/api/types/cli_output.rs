//! CLI command output.

use serde::{Deserialize, Serialize};

/// The output produced by a completed [`crate::api::CliRunner::run`] call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliOutput {
    /// Text written to stdout.
    pub stdout: String,
    /// Text written to stderr.
    pub stderr: String,
    /// Process exit code (0 = success).
    pub exit_code: i32,
}
