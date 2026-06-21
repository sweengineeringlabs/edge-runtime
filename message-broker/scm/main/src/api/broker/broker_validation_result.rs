//! Broker validation result type — rule-161 anchor matching type name `BrokerValidationResult`.

/// A domain-level broker validation result.
///
/// `Ok(())` means the configuration is valid; `Err(String)` carries the
/// human-readable rejection reason.
#[expect(
    dead_code,
    reason = "rule-161 anchor — only in scope within feature-gated tokio-rt paths; downstream crates consume it"
)]
pub type BrokerValidationResult = Result<(), String>;
