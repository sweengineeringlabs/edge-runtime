//! `DefaultRunner` — synchronous facade over [`DaemonRunner`].

use crate::api::Runner;
use crate::api::RuntimeResult;

/// Minimal synchronous [`Runner`] impl for test-time and CLI use.
///
/// In production the async path is used directly via [`DaemonRunner`].
pub(crate) struct DefaultRunner;

impl Runner for DefaultRunner {
    fn run(&self) -> RuntimeResult<()> {
        Ok(())
    }
}
