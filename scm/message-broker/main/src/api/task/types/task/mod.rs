//! Task type group — task unit, identifiers, handle, and factory.

#[allow(clippy::module_inception)]
pub mod task;
pub mod task_handle;
pub mod task_id;
pub mod task_queue_factory;

pub use task::Task;
