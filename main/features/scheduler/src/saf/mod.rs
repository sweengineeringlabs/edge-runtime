//! SAF layer — scheduler public facade.
//!
//! Single entry point: scheduler_svc.rs concentrates all public factory methods on [`SchedulerSvc`].

mod scheduler_svc;

pub use crate::api::error::SchedulerError;
pub use crate::api::scheduler::Scheduler;
pub use crate::api::types::ApplicationConfig;
pub use crate::api::types::ObservabilityConfig;
pub use crate::api::types::SchedulerSvc;
pub use crate::api::types::TracingConfig;
pub use crate::api::validator::Validator;

pub use crate::api::types::SchedulerResult;
#[cfg(feature = "tokio-rt")]
pub use crate::api::types::TokioScheduler;
#[cfg(feature = "tokio-rt")]
pub use crate::api::types::TokioSchedulerConfig;
