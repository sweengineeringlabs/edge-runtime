//! Error type for CLI runner operations.

/// Error type for CLI runner operations.
#[derive(Debug, thiserror::Error)]
pub enum CliError {
    /// Command name was not recognised.
    #[error("command not found: {0}")]
    CommandNotFound(String),
    /// Arguments failed validation.
    #[error("invalid args: {0}")]
    InvalidArgs(String),
    /// Command execution failed.
    #[error("execution failed: {0}")]
    ExecutionFailed(String),
    /// I/O error during execution.
    #[error("io: {0}")]
    Io(String),
}
