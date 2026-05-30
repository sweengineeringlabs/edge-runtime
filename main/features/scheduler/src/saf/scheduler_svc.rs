//! SAF — scheduler public factory methods on [`SchedulerSvc`].

use crate::api::scheduler::Scheduler;
use crate::api::types::SchedulerSvc;
#[cfg(feature = "tokio-rt")]
use crate::api::types::TokioSchedulerConfig;
#[cfg(feature = "tokio-rt")]
use crate::api::validator::Validator;
#[cfg(feature = "tokio-rt")]
use crate::spi::TokioScheduler;

impl SchedulerSvc {
    /// Return a [`ConfigBuilder`] pre-seeded with this crate's package name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        swe_edge_configbuilder::ConfigLoaderFactory::create_config_builder()
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
    }

    /// Validate a value that implements [`Validator`].
    #[cfg(feature = "tokio-rt")]
    pub fn validate<V: Validator>(v: &V) -> Result<(), String> {
        v.validate()
    }

    /// Construct a tokio-backed scheduler with the given config and thread name prefix.
    ///
    /// Use the returned scheduler with [`Scheduler::run`] to drive any async future.
    #[cfg(feature = "tokio-rt")]
    pub fn tokio_scheduler(
        config: TokioSchedulerConfig,
        thread_name: impl Into<String>,
    ) -> impl Scheduler {
        TokioScheduler::new(config, thread_name)
    }
}
