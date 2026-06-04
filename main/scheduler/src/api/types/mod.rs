//! Public value-object types for `swe-edge-runtime-scheduler`.

pub mod application_config;
pub mod observability_config;
pub mod scheduler;
#[cfg(feature = "tokio-rt")]
pub mod tokio;
pub mod tracing_config;

pub use application_config::ApplicationConfig;
pub use observability_config::ObservabilityConfig;
pub use scheduler::SchedulerResult;
pub use scheduler::SchedulerSvc;
#[cfg(feature = "tokio-rt")]
pub use tokio::TokioScheduler;
#[cfg(feature = "tokio-rt")]
pub use tokio::TokioSchedulerConfig;
pub use tracing_config::TracingConfig;
