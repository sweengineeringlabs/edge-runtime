//! Runtime runner interface — async daemon run-loop contract.

/// Maximum duration in seconds the daemon runner will wait for a clean shutdown.
pub const MAX_SHUTDOWN_WAIT_SECS: u64 = 120;
