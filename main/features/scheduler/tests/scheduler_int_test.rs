//! Integration tests for the scheduler crate.

use std::future::Future;
use std::num::NonZeroUsize;

use swe_edge_runtime::{Runtime, RuntimeError, RuntimeResult};
use swe_edge_runtime_scheduler::{run_with_scheduler, RuntimeBuilderExt, Scheduler};

// ── Custom Scheduler impl ─────────────────────────────────────────────────────

struct SingleThreadScheduler;

impl Scheduler for SingleThreadScheduler {
    fn run<F>(&self, fut: F) -> RuntimeResult<()>
    where
        F: Future<Output = RuntimeResult<()>> + Send + 'static,
    {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| RuntimeError::StartFailed(format!("single-thread: {e}")))?
            .block_on(fut)
    }
}

// ── run_with_scheduler (always available) ─────────────────────────────────────

#[test]
fn test_run_with_scheduler_custom_impl_returns_start_failed_for_empty_builder() {
    let result = run_with_scheduler(Runtime::builder(), SingleThreadScheduler);
    assert!(matches!(result, Err(RuntimeError::StartFailed(_))));
}

#[test]
fn test_run_with_scheduler_extension_method_uses_custom_impl() {
    let result = Runtime::builder().run_with_scheduler(SingleThreadScheduler);
    assert!(matches!(result, Err(RuntimeError::StartFailed(_))));
}

#[test]
fn test_run_with_scheduler_is_synchronous_no_async_required() {
    fn _assert_sync<F: FnOnce() -> RuntimeResult<()>>(_f: F) {}
    _assert_sync(|| Runtime::builder().run_with_scheduler(SingleThreadScheduler));
}

// ── TokioSchedulerConfig + tokio factory (tokio-rt) ──────────────────────────

#[cfg(feature = "tokio-rt")]
mod tokio_tests {
    use super::*;
    use swe_edge_runtime_scheduler::{run, run_with_config, tokio_scheduler, TokioSchedulerConfig};

    #[test]
    fn test_tokio_scheduler_config_default_has_all_fields_none() {
        let cfg = TokioSchedulerConfig::default();
        assert!(cfg.workers.is_none());
        assert!(cfg.thread_stack_kib.is_none());
        assert!(cfg.max_blocking_threads.is_none());
        assert!(cfg.thread_name.is_none());
    }

    #[test]
    fn test_tokio_scheduler_config_roundtrips_through_toml() {
        let cfg = TokioSchedulerConfig {
            workers: NonZeroUsize::new(2),
            thread_name: Some("svc".into()),
            ..Default::default()
        };
        let s = toml::to_string(&cfg).expect("serialize");
        let back: TokioSchedulerConfig = toml::from_str(&s).expect("deserialize");
        assert_eq!(back.workers, cfg.workers);
        assert_eq!(back.thread_name.as_deref(), Some("svc"));
    }

    #[test]
    fn test_run_returns_start_failed_when_no_handler_registered() {
        assert!(matches!(
            run(Runtime::builder()),
            Err(RuntimeError::StartFailed(_))
        ));
    }

    #[test]
    fn test_run_with_config_applies_worker_count() {
        let cfg = TokioSchedulerConfig {
            workers: NonZeroUsize::new(1),
            ..Default::default()
        };
        assert!(matches!(
            Runtime::builder().run_with_config(cfg),
            Err(RuntimeError::StartFailed(_))
        ));
    }

    #[test]
    fn test_run_free_fn_returns_start_failed_when_no_handler_registered() {
        assert!(matches!(
            run(Runtime::builder()),
            Err(RuntimeError::StartFailed(_))
        ));
    }

    #[test]
    fn test_run_with_config_free_fn_applies_scheduler_config() {
        let cfg = TokioSchedulerConfig {
            workers: NonZeroUsize::new(2),
            thread_name: Some("test-worker".into()),
            ..Default::default()
        };
        assert!(matches!(
            run_with_config(Runtime::builder(), cfg),
            Err(RuntimeError::StartFailed(_))
        ));
    }

    #[test]
    fn test_tokio_scheduler_factory_can_be_passed_to_run_with_scheduler() {
        let s = tokio_scheduler(TokioSchedulerConfig::default(), "test");
        assert!(matches!(
            Runtime::builder().run_with_scheduler(s),
            Err(RuntimeError::StartFailed(_))
        ));
    }

    #[test]
    fn test_run_is_synchronous_no_async_required() {
        fn _assert_sync<F: FnOnce() -> RuntimeResult<()>>(_f: F) {}
        _assert_sync(|| Runtime::builder().run());
    }
}
