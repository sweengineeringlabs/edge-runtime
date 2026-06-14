//! Broker validation result — rule-121 counterpart for `core/broker/validator`.

/// Result type for broker-layer validation.
///
/// `Ok(())` means the configuration is valid; `Err(String)` carries an
/// actionable rejection reason.
#[expect(
    dead_code,
    reason = "rule-121 anchor for core/broker/validator — no live callers in this crate; available to downstream"
)]
pub type Validator = Result<(), String>;
