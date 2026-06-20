//! Testable daemon runner — start, await signal, shut down within timeout.

mod default_runner;
#[allow(clippy::module_inception)]
mod runner;

pub(crate) use default_runner::DefaultRunner;
pub(crate) use runner::DaemonRunner;
