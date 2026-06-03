//! [`TokioSchedulerConfigBuilder`] — fluent builder for [`TokioSchedulerConfig`].

use std::num::NonZeroUsize;

use super::tokio_scheduler_config::TokioSchedulerConfig;

/// Fluent builder for [`TokioSchedulerConfig`].
///
/// Construct via [`TokioSchedulerConfigBuilder::new`], chain the setter methods,
/// then call [`build`](TokioSchedulerConfigBuilder::build) to obtain a [`TokioSchedulerConfig`].
#[derive(Debug, Default)]
pub struct TokioSchedulerConfigBuilder {
    config: TokioSchedulerConfig,
}

impl TokioSchedulerConfigBuilder {
    /// Create a new builder with all-default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the number of worker threads.
    pub fn workers(mut self, n: NonZeroUsize) -> Self {
        self.config.workers = Some(n);
        self
    }

    /// Set the stack size per worker thread in KiB.
    pub fn thread_stack_kib(mut self, kib: usize) -> Self {
        self.config.thread_stack_kib = Some(kib);
        self
    }

    /// Set the maximum number of blocking-pool threads.
    pub fn max_blocking_threads(mut self, n: usize) -> Self {
        self.config.max_blocking_threads = Some(n);
        self
    }

    /// Set the worker thread name prefix.
    pub fn thread_name(mut self, name: impl Into<String>) -> Self {
        self.config.thread_name = Some(name.into());
        self
    }

    /// Consume the builder and return the finished [`TokioSchedulerConfig`].
    pub fn build(self) -> TokioSchedulerConfig {
        self.config
    }
}
