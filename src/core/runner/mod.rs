//! Testable daemon runner — start, await signal, shut down within timeout.

mod runner;

pub(crate) use runner::run_until_signal;
