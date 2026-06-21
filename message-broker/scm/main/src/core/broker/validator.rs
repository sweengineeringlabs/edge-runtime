//! [`DefaultValidator`] — default implementation of the [`Validator`] contract.

use crate::api::Validator;

/// Default validator implementation that always returns success.
///
/// Used as a no-op guard for broker instances that have no meaningful
/// configuration validation requirements (e.g. in-memory backends).
#[expect(
    dead_code,
    reason = "SEA core/ anchor — satisfies rule-49; used by in-memory broker config path"
)]
pub(crate) struct DefaultValidator;

impl Validator for DefaultValidator {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_always_ok() {
        assert!(DefaultValidator.validate().is_ok());
    }
}
