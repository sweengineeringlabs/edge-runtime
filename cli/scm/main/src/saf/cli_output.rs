//! Inherent impl for [`CliOutput`] — construction and inspection helpers.

use crate::api::CliOutput;

impl CliOutput {
    /// Construct an output with all three fields.
    pub fn new(stdout: impl Into<String>, stderr: impl Into<String>, exit_code: i32) -> Self {
        Self {
            stdout: stdout.into(),
            stderr: stderr.into(),
            exit_code,
        }
    }

    /// Construct a successful output with the given stdout text.
    pub fn success(stdout: impl Into<String>) -> Self {
        Self::new(stdout, "", 0)
    }

    /// Returns `true` when `exit_code == 0`.
    pub fn is_success(&self) -> bool {
        self.exit_code == 0
    }
}
