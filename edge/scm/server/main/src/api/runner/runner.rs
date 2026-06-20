//! Runner interface — mirrors `core/runner`.

pub use crate::api::runtime::traits::runner::Runner;

/// Timeout in seconds for the run-to-completion cycle before aborting.
pub const DEFAULT_RUN_TIMEOUT_SECS: u64 = 30;
