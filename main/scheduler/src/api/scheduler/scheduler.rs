//! [`Scheduler`] — runtime-agnostic contract for driving an async future.

use std::future::Future;

use crate::api::error::SchedulerError;

/// Drives an async future to completion on the caller's chosen async runtime.
///
/// Implement this trait to plug in any runtime — tokio, async-std, smol, or a
/// custom executor. The crate ships [`crate::TokioScheduler`] as a ready-made
/// implementation behind the `tokio-rt` feature (enabled by default).
///
/// The scheduler only fails with [`SchedulerError`] if the async runtime itself
/// cannot be initialised. The future's own result is passed through unchanged.
pub trait Scheduler {
    /// Block the calling thread until `fut` completes and return its output.
    ///
    /// Returns `Err(SchedulerError)` only if the scheduler's async runtime
    /// fails to start. Otherwise returns `Ok(fut.await)`.
    fn run<F, T>(&self, fut: F) -> Result<T, SchedulerError>
    where
        F: Future<Output = T> + Send + 'static;
}
