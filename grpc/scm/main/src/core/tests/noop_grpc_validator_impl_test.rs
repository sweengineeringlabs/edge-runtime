//! Colocated tests for the `NoopGrpcValidator` core implementation of `Validator`.

#[cfg(test)]
mod tests {
    use crate::api::{NoopGrpcValidator, Validator};

    /// @covers: validate
    #[test]
    fn test_validate_noop_returns_unit_ok_happy() {
        // The no-op validator accepts unconditionally and yields the unit payload
        // — asserting the exact Ok(()) shape, not merely is_ok().
        let validator = NoopGrpcValidator;
        assert!(
            matches!(validator.validate(), Ok(())),
            "NoopGrpcValidator::validate must return Ok(())"
        );
    }

    /// @covers: validate
    #[test]
    fn test_validate_noop_is_idempotent_across_calls_edge() {
        // Repeated validation must stay exactly Ok(()) — the noop carries no state
        // that could flip the payload or variant on a later call.
        let validator = NoopGrpcValidator;
        assert!(
            matches!(validator.validate(), Ok(())),
            "first call must be Ok(())"
        );
        assert!(
            matches!(validator.validate(), Ok(())),
            "second call must also be Ok(())"
        );
    }
}
