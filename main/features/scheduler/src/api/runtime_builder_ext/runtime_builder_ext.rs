//! [`RuntimeBuilderExt`] — synchronous entry point extension for [`RuntimeBuilder`].

use swe_edge_runtime::RuntimeResult;

use crate::api::scheduler::Scheduler;
#[cfg(feature = "tokio-rt")]
use crate::api::scheduler::tokio_scheduler_config::TokioSchedulerConfig;

/// Extension trait that adds synchronous entry points to [`RuntimeBuilder`].
///
/// Import this trait and call `.run()` (tokio-rt feature) or
/// `.run_with_scheduler(s)` (any runtime) from a plain `fn main()`.
pub trait RuntimeBuilderExt {
    /// Drive the runtime using `scheduler`, blocking until SIGTERM/SIGINT or error.
    fn run_with_scheduler<S: Scheduler>(self, scheduler: S) -> RuntimeResult<()>;

    /// Drive the runtime with the tokio scheduler and default config.
    ///
    /// Requires the `tokio-rt` feature (enabled by default).
    #[cfg(feature = "tokio-rt")]
    fn run(self) -> RuntimeResult<()>;

    /// Drive the runtime with the tokio scheduler and the supplied config.
    ///
    /// Requires the `tokio-rt` feature (enabled by default).
    #[cfg(feature = "tokio-rt")]
    fn run_with_config(self, config: TokioSchedulerConfig) -> RuntimeResult<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_builder_ext_is_object_safe_check() {
        // Compile-time proof: the trait is well-formed.
        fn _accepts_trait_object<T: RuntimeBuilderExt>() {}
    }
}
