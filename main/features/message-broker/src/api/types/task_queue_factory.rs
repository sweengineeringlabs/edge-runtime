//! [`TaskQueueFactory`] — zero-size factory type for constructing task queue instances.

/// Zero-size factory type for constructing task queue instances.
///
/// All factory methods are associated functions on this type.
/// Consumers never construct `TaskQueueFactory` directly — they call
/// associated functions like `TaskQueueFactory::in_memory()`.
pub struct TaskQueueFactory;
