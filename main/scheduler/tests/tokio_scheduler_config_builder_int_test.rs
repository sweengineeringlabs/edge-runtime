//! Integration tests for [`TokioSchedulerConfigBuilder`].

use swe_edge_runtime_scheduler::{TokioSchedulerConfig, TokioSchedulerConfigBuilder};

/// @covers: TokioSchedulerConfigBuilder::build
#[test]
fn test_build_returns_default_config_when_no_options_set() {
    let config = TokioSchedulerConfigBuilder::new().build();
    assert_eq!(config, TokioSchedulerConfig::default());
}

/// @covers: TokioSchedulerConfigBuilder::thread_name
#[test]
fn test_thread_name_is_stored_in_config() {
    let config = TokioSchedulerConfigBuilder::new()
        .thread_name("swe-worker")
        .build();
    assert_eq!(config.thread_name.as_deref(), Some("swe-worker"));
}

/// @covers: TokioSchedulerConfigBuilder::max_blocking_threads
#[test]
fn test_max_blocking_threads_is_stored_in_config() {
    let config = TokioSchedulerConfigBuilder::new()
        .max_blocking_threads(32)
        .build();
    assert_eq!(config.max_blocking_threads, Some(32));
}
