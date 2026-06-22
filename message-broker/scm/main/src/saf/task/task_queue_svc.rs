//! SAF — [`TaskQueue`] implementation.

/// Maximum task payload size in bytes accepted by the default in-memory queue.
#[expect(dead_code)]
pub const MAX_TASK_PAYLOAD_BYTES: usize = 4 * 1024 * 1024;
