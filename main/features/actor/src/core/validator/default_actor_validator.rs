//! Default validator for actor mailbox configuration.

use crate::api::traits::Validator;

/// Default validator for actor mailbox configuration.
///
/// Validates that actor mailbox invariants hold before actors are spawned.
/// This is the built-in implementation; consumers may supply their own via
/// the `Validator` trait.
pub(crate) struct DefaultActorValidator {
    /// Mailbox capacity configured for this actor.
    capacity: usize,
}

impl DefaultActorValidator {
    /// Create a validator for the given mailbox capacity.
    #[expect(
        dead_code,
        reason = "SEA core/ anchor — wired up when validator integrates into spawn"
    )]
    pub(crate) fn new(capacity: usize) -> Self {
        Self { capacity }
    }
}

impl Validator for DefaultActorValidator {
    /// Validate that the mailbox capacity is non-zero.
    ///
    /// Returns `Err` if capacity is zero (would deadlock all senders immediately).
    fn validate(&self) -> Result<(), String> {
        if self.capacity == 0 {
            return Err(
                "actor mailbox capacity must be greater than zero — zero capacity deadlocks senders"
                    .to_owned(),
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_creates_validator_with_given_capacity() {
        let v = DefaultActorValidator::new(42);
        // Positive capacity — should validate successfully
        assert!(v.validate().is_ok(), "capacity 42 must be valid");
    }

    /// @covers: validate
    #[test]
    fn test_validate_nonzero_capacity_passes() {
        let v = DefaultActorValidator::new(1024);
        assert!(v.validate().is_ok());
    }

    /// @covers: validate
    #[test]
    fn test_validate_zero_capacity_fails_with_message() {
        let v = DefaultActorValidator::new(0);
        let err = v.validate().unwrap_err();
        assert!(
            err.contains("greater than zero"),
            "error should mention capacity requirement"
        );
    }
}
