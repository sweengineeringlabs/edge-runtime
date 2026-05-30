//! Integration tests for [`SchedulerSvc`] entry point.

use swe_edge_runtime_scheduler::SchedulerSvc;

/// @covers: SchedulerSvc::create_config_builder
#[test]
fn test_scheduler_svc_create_config_builder_returns_builder() {
    let _builder = SchedulerSvc::create_config_builder();
}

/// @covers: SchedulerSvc::validate
#[cfg(feature = "tokio-rt")]
#[test]
fn test_scheduler_svc_validate_accepts_valid_config() {
    use swe_edge_runtime_scheduler::TokioSchedulerConfig;
    assert!(SchedulerSvc::validate(&TokioSchedulerConfig::default()).is_ok());
}
