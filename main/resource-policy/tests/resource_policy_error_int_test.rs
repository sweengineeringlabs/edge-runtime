//! Integration tests for ResourcePolicyError.

use swe_edge_runtime_resource_policy::ResourcePolicyError;

/// @covers: ResourcePolicyError::UnknownPolicy
#[test]
fn test_unknown_policy_error_message_contains_name() {
    let e = ResourcePolicyError::UnknownPolicy {
        name: "ghost".into(),
    };
    assert!(e.to_string().contains("ghost"));
}

/// @covers: ResourcePolicyError::ConfigParse
#[test]
fn test_config_parse_error_message_contains_reason() {
    let e = ResourcePolicyError::ConfigParse {
        reason: "bad syntax".into(),
    };
    assert!(e.to_string().contains("bad syntax"));
}

/// @covers: ResourcePolicyError::InvalidValue
#[test]
fn test_invalid_value_error_message_contains_field_and_value() {
    let e = ResourcePolicyError::InvalidValue {
        field: "timeout_ms".into(),
        value: 999,
        reason: "exceeds maximum".into(),
    };
    let msg = e.to_string();
    assert!(msg.contains("timeout_ms"));
    assert!(msg.contains("999"));
}
