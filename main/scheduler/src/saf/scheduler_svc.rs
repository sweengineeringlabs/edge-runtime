//! SAF — scheduler public factory methods on [`SchedulerSvc`].

use crate::api::types::SchedulerSvc;
#[cfg(feature = "tokio-rt")]
use crate::api::types::TokioScheduler;
#[cfg(feature = "tokio-rt")]
use crate::api::types::TokioSchedulerConfig;
#[cfg(feature = "tokio-rt")]
use crate::api::validator::Validator;

impl SchedulerSvc {
    /// Return a [`ConfigBuilder`] pre-seeded with this crate's package name and version.
    pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
        let mut b = swe_edge_configbuilder::ConfigBuilderImpl::new();
        b = b.with_name(env!("CARGO_PKG_NAME"));
        b = b.with_version(env!("CARGO_PKG_VERSION"));
        b
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
    ) -> TokioScheduler {
        let thread_name = config
            .thread_name
            .clone()
            .unwrap_or_else(|| thread_name.into());
        TokioScheduler {
            config,
            thread_name,
        }
    }
}
