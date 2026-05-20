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

/// Fluent builder for [`TokioSchedulerConfig`].
#[allow(dead_code)]
struct TokioSchedulerConfigBuilder {
    inner: TokioSchedulerConfig,
}

#[allow(dead_code)]
impl TokioSchedulerConfigBuilder {
    fn new() -> Self {
        Self {
            inner: TokioSchedulerConfig::default(),
        }
    }
    fn workers(mut self, n: NonZeroUsize) -> Self {
        self.inner.workers = Some(n);
        self
    }
    fn thread_stack_kib(mut self, kib: usize) -> Self {
        self.inner.thread_stack_kib = Some(kib);
        self
    }
    fn max_blocking_threads(mut self, n: usize) -> Self {
        self.inner.max_blocking_threads = Some(n);
        self
    }
    fn thread_name(mut self, name: impl Into<String>) -> Self {
        self.inner.thread_name = Some(name.into());
        self
    }
    fn build(self) -> TokioSchedulerConfig {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_has_all_fields_none() {
        let cfg = TokioSchedulerConfig::default();
        assert!(cfg.workers.is_none());
        assert!(cfg.thread_stack_kib.is_none());
        assert!(cfg.max_blocking_threads.is_none());
        assert!(cfg.thread_name.is_none());
    }

    #[test]
    fn test_roundtrips_through_toml() {
        let cfg = TokioSchedulerConfig {
            workers: NonZeroUsize::new(4),
            thread_name: Some("svc".into()),
            ..Default::default()
        };
        let s = toml::to_string(&cfg).expect("serialize");
        let back: TokioSchedulerConfig = toml::from_str(&s).expect("deserialize");
        assert_eq!(back.workers, cfg.workers);
        assert_eq!(back.thread_name.as_deref(), Some("svc"));
    }

    #[test]
    fn test_deserializes_from_empty_toml() {
        let cfg: TokioSchedulerConfig = toml::from_str("").expect("empty toml");
        assert!(cfg.workers.is_none());
    }
}
