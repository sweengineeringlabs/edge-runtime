//! Integration tests for ProfileSpec.

use swe_edge_runtime_isolator::ProfileSpec;

/// @covers: ProfileSpec::default_kill_on_job_close
#[test]
fn test_profile_spec_default_kill_on_job_close_is_true() {
    assert!(ProfileSpec::default_kill_on_job_close());
}

/// @covers: ProfileSpec
#[test]
fn test_profile_spec_deserializes_noop() {
    let toml = r#"kind = "noop""#;
    let spec: ProfileSpec = toml::from_str(toml).expect("valid TOML");
    assert_eq!(spec.kind, "noop");
    assert!(spec.kill_on_job_close); // default is true
}
