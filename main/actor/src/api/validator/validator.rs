//! Interface contract for the default actor validator.

/// Validates that an actor configuration is in a legal state before use.
#[expect(
    dead_code,
    reason = "SEA api/ interface anchor (Rule 121) — intentionally unused"
)]
pub trait Validator {
    /// Return `Ok(())` when valid, or `Err` with a description of the failure.
    fn validate(&self) -> Result<(), String>;
}
