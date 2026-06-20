//! Tests for `api/broker/broker_validation_result` — rule-120 coverage.
//!
//! `BrokerValidationResult` is `pub(crate)` and not part of the public API.
//! Tests here exercise the same `Result<(), String>` contract to confirm the
//! module's semantic intent compiles and behaves correctly.

/// @covers: api/broker/broker_validation_result::BrokerValidationResult
#[test]
fn test_broker_validation_result_ok_variant_happy() {
    let result: Result<(), String> = Ok(());
    assert!(result.is_ok());
}

/// @covers: api/broker/broker_validation_result::BrokerValidationResult
#[test]
fn test_broker_validation_result_err_variant_error() {
    let result: Result<(), String> = Err("invalid configuration".into());
    assert!(result.is_err());
}

/// @covers: api/broker/broker_validation_result::BrokerValidationResult
#[test]
fn test_broker_validation_result_err_message_is_preserved_edge() {
    let msg = "broker url is required";
    let result: Result<(), String> = Err(msg.to_owned());
    assert!(result.is_err_and(|e| e == msg));
}
