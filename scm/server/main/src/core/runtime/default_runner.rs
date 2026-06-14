//! `DefaultRunner` — synchronous facade over [`DaemonRunner`].

use crate::api::runtime::errors::runtime_result::RuntimeResult;
use crate::api::runtime::traits::runner::Runner;

/// Minimal synchronous [`Runner`] impl for test-time and CLI use.
///
/// In production the async path is used directly via [`DaemonRunner`].
pub(crate) struct DefaultRunner;

impl Runner for DefaultRunner {
    fn run(&self) -> RuntimeResult<()> {
        Ok(())
    }
}
