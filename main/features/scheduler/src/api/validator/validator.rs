//! [`Validator`] — validates that a value is in a legal state before use.

/// Validates that a value is in a legal state before use.
pub trait Validator {
    /// Return `Ok(())` when the value is valid, or `Err` with an actionable
    /// description of the first validation failure.
    fn validate(&self) -> Result<(), String>;
}
