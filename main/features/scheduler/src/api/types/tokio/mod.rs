//! Tokio-backed scheduler types.

#[cfg(feature = "tokio-rt")]
pub mod tokio_scheduler;
#[cfg(feature = "tokio-rt")]
pub mod tokio_scheduler_config;
#[cfg(feature = "tokio-rt")]
pub mod tokio_scheduler_config_builder;

#[cfg(feature = "tokio-rt")]
pub use tokio_scheduler::TokioScheduler;
#[cfg(feature = "tokio-rt")]
pub use tokio_scheduler_config::TokioSchedulerConfig;
#[cfg(feature = "tokio-rt")]
#[expect(
    unused_imports,
    reason = "SEA api/ anchor — TokioSchedulerConfigBuilder exported for consumers, not used internally"
)]
pub use tokio_scheduler_config_builder::TokioSchedulerConfigBuilder;
