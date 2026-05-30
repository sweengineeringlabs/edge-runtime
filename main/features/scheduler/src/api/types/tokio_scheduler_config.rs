//! [`TokioSchedulerConfig`] — tokio-specific tuning knobs for the async executor.

use std::num::NonZeroUsize;

use serde::{Deserialize, Serialize};

/// Tuning configuration for the tokio-backed scheduler.
///
/// All fields are optional; absent fields let tokio pick its own defaults.
/// Deserializes from the `[scheduler]` section of your application's TOML config.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct TokioSchedulerConfig {
    /// Number of worker threads (default: logical CPU count).
    pub workers: Option<NonZeroUsize>,
    /// Stack size per worker thread in KiB (default: OS default, ~8 MiB).
    pub thread_stack_kib: Option<usize>,
    /// Maximum threads in the blocking pool (default: 512).
    pub max_blocking_threads: Option<usize>,
    /// Worker thread name prefix (visible in `ps` and profilers).
    pub thread_name: Option<String>,
}
