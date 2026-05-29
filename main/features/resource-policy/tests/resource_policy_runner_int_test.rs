//! Integration tests for ResourcePolicyRunner.

use std::sync::Arc;

use swe_edge_egress_subprocess::{SubprocessArgs, SubprocessResult, SubprocessRunner as _, SubprocessSvc};
use swe_edge_runtime_resource_policy::{PolicySvc, ResourcePolicy};

fn test_policy() -> ResourcePolicy {
    ResourcePolicy {
        name: "test".into(),
        timeout_ms: 5_000,
        output_bytes_cap: 65_536,
        cpu_time_ms: 0,
        memory_bytes: 0,
    }
}

/// @covers: ResourcePolicyRunner::policy
#[test]
fn test_resource_policy_runner_policy_is_accessible() {
    let runner = PolicySvc::create_policy_runner(Arc::new(SubprocessSvc::runner()), test_policy());
    assert_eq!(runner.policy().name, "test");
}

/// @covers: SubprocessRunner for ResourcePolicyRunner
#[tokio::test]
async fn test_resource_policy_runner_run_returns_spawn_failed_for_nonexistent_binary() {
    let runner = PolicySvc::create_policy_runner(Arc::new(SubprocessSvc::runner()), test_policy());
    let args = SubprocessArgs::builder()
        .argv(vec!["__nonexistent__".into()])
        .allow_commands(vec!["__nonexistent__".into()])
        .build();
    let result = runner.run(args).await;
    assert!(matches!(result, SubprocessResult::SpawnFailed { .. }));
}
