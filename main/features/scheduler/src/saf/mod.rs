//! SAF layer — scheduler public facade.
//!
//! Single entry point: edge_runtime_scheduler_svc.rs concentrates all public factories.

mod edge_runtime_scheduler_svc;

pub use crate::api::application_config_builder::ApplicationConfigBuilder;
pub use crate::api::scheduler::{Scheduler, SchedulerError, SchedulerResult};
pub use crate::api::traits::Validator;

#[cfg(feature = "tokio-rt")]
pub use crate::api::scheduler::TokioSchedulerConfig;
#[cfg(feature = "tokio-rt")]
pub use edge_runtime_scheduler_svc::{tokio_scheduler, validate};
