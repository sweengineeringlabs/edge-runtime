//! Integration tests for [`TokioSchedulerConfig`].

use swe_edge_runtime_scheduler::TokioSchedulerConfig;

/// @covers: TokioSchedulerConfig
#[test]
fn test_tokio_scheduler_config_default_has_all_fields_none() {
    let cfg = TokioSchedulerConfig::default();
    assert!(cfg.workers.is_none());
    assert!(cfg.thread_stack_kib.is_none());
    assert!(cfg.max_blocking_threads.is_none());
    assert!(cfg.thread_name.is_none());
}

/// @covers: TokioSchedulerConfig
#[test]
fn test_tokio_scheduler_config_roundtrips_through_toml() {
    use std::num::NonZeroUsize;
    let cfg = TokioSchedulerConfig {
        workers: NonZeroUsize::new(4),
        thread_name: Some("svc".into()),
        ..Default::default()
    };
    let s = toml::to_string(&cfg)
        .map_err(|e| e.to_string())
        .unwrap_or_default();
    let back: TokioSchedulerConfig = toml::from_str(&s)
        .map_err(|e| e.to_string())
        .unwrap_or_default();
    assert_eq!(back.workers, cfg.workers);
}
