//! Validator trait for actor configuration and state validation.

/// Validator trait for actor mailbox configuration and state validation.
///
/// Implementations must validate actor configurations and state invariants
/// before and after actor initialization.
#[expect(dead_code, reason = "SEA api/ anchor — exported for consumers, not used internally")]
pub trait Validator {
    /// Validate actor mailbox configuration or state.
    ///
    /// Returns `Ok(())` if valid, or an error message if validation fails.
    fn validate(&self) -> Result<(), String>;
}
