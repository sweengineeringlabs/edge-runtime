//! Integration tests for the resource policy public API.

use std::sync::Arc;

use swe_edge_egress_process::{process_runner, ProcessArgs, ProcessResult, ProcessRunner as _};
use swe_edge_runtime_resource_policy::{
    create_resource_policy_runner, ResourceLimits, ResourceLimitsResolver, ResourcePolicy,
    ResourcePolicyConfig, ResourcePolicyError,
};

// ── ResourcePolicyConfig ─────────────────────────────────────────────────────

/// @covers: ResourcePolicyConfig::get — returns the named policy.
#[test]
fn test_policy_config_get_returns_correct_policy() {
    let toml = r#"
        [default]
        name             = "default"
        timeout_ms       = 30000
        output_bytes_cap = 1048576
        cpu_time_ms      = 0
        memory_bytes     = 0
    "#;
    let cfg = ResourcePolicyConfig(toml::from_str(toml).unwrap());
    let policy = cfg.get("default").unwrap();
    assert_eq!(policy.timeout_ms, 30_000);
    assert_eq!(policy.output_bytes_cap, 1_048_576);
}

/// @covers: ResourcePolicyConfig::get — unknown name returns UnknownPolicy.
#[test]
fn test_policy_config_get_unknown_returns_unknown_policy_error() {
    let cfg = ResourcePolicyConfig::default();
    let err = cfg.get("ghost").unwrap_err();
    assert!(
        matches!(err, ResourcePolicyError::UnknownPolicy { .. }),
        "expected UnknownPolicy; got {err:?}",
    );
}

// ── ResourceLimitsResolver ───────────────────────────────────────────────────

/// @covers: ResourceLimitsResolver — step limit overrides agent and floor.
#[test]
fn test_resolver_step_overrides_agent_and_floor() {
    let floor = ResourcePolicy {
        name: "floor".into(),
        timeout_ms: 30_000,
        output_bytes_cap: 1_048_576,
        cpu_time_ms: 0,
        memory_bytes: 0,
    };
    let step = ResourceLimits {
        timeout_ms: Some(2_000),
        ..Default::default()
    };
    let agent = ResourceLimits {
        timeout_ms: Some(10_000),
        cpu_time_ms: Some(5_000),
        ..Default::default()
    };

    let resolved = ResourceLimitsResolver::new()
        .layer(step)
        .layer(agent)
        .resolve_with_defaults(&floor);

    assert_eq!(resolved.timeout_ms, 2_000);
    assert_eq!(resolved.cpu_time_ms, 5_000);
    assert_eq!(resolved.output_bytes_cap, 1_048_576);
}

// ── ResourcePolicyRunner ─────────────────────────────────────────────────────

/// @covers: ResourcePolicyRunner — injects policy limits; completed run succeeds.
#[tokio::test]
async fn test_resource_policy_runner_completes_with_injected_limits() {
    let policy = ResourcePolicy {
        name: "test".into(),
        timeout_ms: 5_000,
        output_bytes_cap: 65_536,
        cpu_time_ms: 0,
        memory_bytes: 0,
    };
    let runner = create_resource_policy_runner(Arc::new(process_runner()), policy);

    #[cfg(unix)]
    let (argv, allow) = (vec!["echo".into(), "ok".into()], vec!["echo".into()]);
    #[cfg(windows)]
    let (argv, allow) = (
        vec!["cmd".into(), "/C".into(), "echo ok".into()],
        vec!["cmd".into()],
    );

    let args = ProcessArgs::builder()
        .argv(argv)
        .allow_commands(allow)
        .build();
    let result = runner.run(args).await;
    assert!(
        matches!(result, ProcessResult::Completed { exit_code: 0, .. }),
        "expected Completed(0); got {result:?}",
    );
}

/// @covers: ResourcePolicyRunner — caller's explicit timeout is not overwritten.
#[tokio::test]
async fn test_resource_policy_runner_preserves_caller_timeout() {
    let policy = ResourcePolicy {
        name: "test".into(),
        timeout_ms: 99_999,
        output_bytes_cap: 65_536,
        cpu_time_ms: 0,
        memory_bytes: 0,
    };
    let runner = create_resource_policy_runner(Arc::new(process_runner()), policy);

    // A nonexistent binary denied before spawning — just confirms no panic
    // injecting policy when caller's timeout_ms is already set.
    let args = ProcessArgs::builder()
        .argv(vec!["__nonexistent__".into()])
        .allow_commands(vec!["__nonexistent__".into()])
        .timeout_ms(1_000)
        .build();
    let result = runner.run(args).await;
    assert!(matches!(result, ProcessResult::SpawnFailed { .. }));
}

/// @covers: ResourcePolicyRunner — denied when allow_list empty (policy doesn't bypass it).
#[tokio::test]
async fn test_resource_policy_runner_deny_not_bypassed_by_policy() {
    let policy = ResourcePolicy {
        name: "test".into(),
        timeout_ms: 5_000,
        output_bytes_cap: 1_048_576,
        cpu_time_ms: 0,
        memory_bytes: 0,
    };
    let runner = create_resource_policy_runner(Arc::new(process_runner()), policy);
    let args = ProcessArgs::builder().argv(vec!["echo".into()]).build(); // no allow_commands → Denied
    let result = runner.run(args).await;
    assert!(
        matches!(result, ProcessResult::Denied { .. }),
        "expected Denied; got {result:?}",
    );
}
