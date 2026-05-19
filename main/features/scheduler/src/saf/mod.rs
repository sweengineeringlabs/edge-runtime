//! SAF layer — scheduler public facade.

mod edge_runtime_scheduler_svc;

pub use crate::api::application_config_builder::ApplicationConfigBuilder;
#[cfg(feature = "tokio-rt")]
pub use crate::api::scheduler::TokioSchedulerConfig;
#[cfg(feature = "tokio-rt")]
pub use edge_runtime_scheduler_svc::{tokio_scheduler, validate};
