//! SEA interface contract — primary traits and services.

/// Validator trait for actor mailbox configuration and state validation.
///
/// Implementations must validate actor configurations and state invariants
/// before and after actor initialization.
#[allow(dead_code)]
pub trait Validator {
    /// Validate actor mailbox configuration or state.
    ///
    /// Returns `Ok(())` if valid, or an error message if validation fails.
    fn validate(&self) -> Result<(), String>;
}
