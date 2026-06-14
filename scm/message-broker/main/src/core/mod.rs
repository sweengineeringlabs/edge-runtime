//! Core layer — broker and task queue implementation constants.
//!
//! All items in this layer are `pub(crate)` — they are not part of the
//! public contract and may change between minor versions.

pub(crate) mod broker;
pub(crate) mod config;
pub(crate) mod task;

/// Maximum byte length of a message topic shared across broker and queue backends.
#[expect(
    dead_code,
    reason = "SEA core/ anchor — used by backends, dead when tokio-rt is disabled"
)]
pub(crate) const MAX_TOPIC_LEN: usize = broker::MAX_TOPIC_BYTES;

/// Default channel buffer shared across in-memory backends.
#[expect(
    dead_code,
    reason = "SEA core/ anchor — used by backends, dead when tokio-rt is disabled"
)]
pub(crate) const DEFAULT_CAPACITY: usize = broker::DEFAULT_CHANNEL_CAPACITY;
#[expect(
    dead_code,
    reason = "SEA core/ anchor — used by backends, dead when tokio-rt is disabled"
)]
pub(crate) const DEFAULT_TASK_CAPACITY: usize = task::DEFAULT_TASK_QUEUE_CAPACITY;
