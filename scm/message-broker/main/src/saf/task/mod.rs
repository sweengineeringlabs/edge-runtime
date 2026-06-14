//! SAF task sub-module — task queue surface and factory contract.

pub mod task_queue_factory_contract_svc;
pub mod task_queue_svc;

pub use task_queue_factory_contract_svc::TaskQueueFactoryContract;
#[cfg(feature = "tokio-rt")]
pub use task_queue_svc::InMemoryTaskQueue;
pub use task_queue_svc::QueueError;
pub use task_queue_svc::Task;
pub use task_queue_svc::TaskHandle;
pub use task_queue_svc::TaskId;
pub use task_queue_svc::TaskQueue;
pub use task_queue_svc::TaskQueueFactory;
