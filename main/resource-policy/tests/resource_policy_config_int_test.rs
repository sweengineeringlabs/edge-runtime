//! Integration tests for ResourcePolicyConfig.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use swe_edge_configbuilder::ConfigSection as _;
use swe_edge_runtime_resource_policy::{ResourcePolicyConfig, ResourcePolicyError};

/// @covers: ResourcePolicyConfig::section_name
#[test]
fn test_resource_policy_config_section_name_is_resource_policies() {
    assert_eq!(ResourcePolicyConfig::section_name(), "resource_policies");
}

/// @covers: ResourcePolicyConfig::get
#[test]
fn test_resource_policy_config_get_existing_returns_policy() {
    let toml = r#"
[default]
name             = "default"
timeout_ms       = 30000
output_bytes_cap = 1048576
cpu_time_ms      = 0
memory_bytes     = 0
"#;
    let cfg = ResourcePolicyConfig(toml::from_str(toml).expect("valid toml"));
    let policy = cfg.get("default").expect("default policy");
    assert_eq!(policy.timeout_ms, 30_000);
}

/// @covers: ResourcePolicyConfig::get
#[test]
fn test_resource_policy_config_get_unknown_returns_unknown_policy_error() {
    let cfg = ResourcePolicyConfig::default();
    let err = cfg.get("ghost").unwrap_err();
    assert!(matches!(err, ResourcePolicyError::UnknownPolicy { .. }));
}

/// @covers: ResourcePolicyConfig::is_empty
#[test]
fn test_resource_policy_config_default_is_empty() {
    assert!(ResourcePolicyConfig::default().is_empty());
}

/// @covers: ResourcePolicyConfig::len
#[test]
fn test_resource_policy_config_len_counts_loaded_policies() {
    let toml = r#"
[p1]
name             = "p1"
timeout_ms       = 1000
output_bytes_cap = 512
cpu_time_ms      = 0
memory_bytes     = 0
"#;
    let cfg = ResourcePolicyConfig(toml::from_str(toml).expect("valid toml"));
    assert_eq!(cfg.len(), 1);
}
