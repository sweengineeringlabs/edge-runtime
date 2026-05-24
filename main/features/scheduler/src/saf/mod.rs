//! SAF layer — scheduler public facade.
//!
//! Single entry point: edge_runtime_scheduler_svc.rs concentrates all public factories.

mod edge_runtime_scheduler_svc;

pub use crate::api::scheduler::{Scheduler, SchedulerError, SchedulerResult};
pub use crate::api::traits::Validator;

#[cfg(feature = "tokio-rt")]
pub use crate::api::scheduler::TokioSchedulerConfig;
pub use edge_runtime_scheduler_svc::create_config_builder;
#[cfg(feature = "tokio-rt")]
pub use edge_runtime_scheduler_svc::{tokio_scheduler, validate};
