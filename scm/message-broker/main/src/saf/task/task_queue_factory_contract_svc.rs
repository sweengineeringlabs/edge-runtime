//! SAF — [`TaskQueueFactoryContract`] public service surface.
//!
//! Exposes the [`TaskQueueFactoryContract`] trait for consumers that need to
//! construct task queue instances through the factory contract.

pub use crate::api::task::traits::task_queue_factory_contract::TaskQueueFactoryContract;

/// Identifier for the default task queue factory contract implementation.
pub const TASK_QUEUE_FACTORY_CONTRACT_ID: &str = "task_queue";
