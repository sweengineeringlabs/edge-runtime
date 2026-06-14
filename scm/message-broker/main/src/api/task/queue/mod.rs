//! Task queue interface — mirrors `core/task/queue` for rule-121 pairing.

/// Maximum number of in-flight tasks the default in-memory queue accepts.
///
/// Used as the channel capacity in [`TaskQueueFactory::in_memory`] (tokio-rt feature).
pub const MAX_QUEUE_DEPTH: usize = 16_384;
