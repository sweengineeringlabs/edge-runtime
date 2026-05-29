//! `ResourcePolicyError` — errors from policy loading and resolution.

/// Errors from loading or resolving a [`ResourcePolicy`].
///
/// [`ResourcePolicy`]: super::policy::ResourcePolicy
#[derive(Debug, thiserror::Error)]
pub enum ResourcePolicyError {
    /// The named policy is absent from `subprocess_policy.toml`.
    #[error("resource policy '{name}' not found in subprocess_policy.toml")]
    UnknownPolicy {
        /// The policy name that was requested.
        name: String,
    },

    /// The config section could not be parsed.
    #[error("resource policy config parse error: {reason}")]
    ConfigParse {
        /// The underlying parse error message.
        reason: String,
    },

    /// A policy field value is invalid.
    #[error("resource policy field '{field}' has invalid value {value}: {reason}")]
    InvalidValue {
        /// The field name.
        field: String,
        /// The invalid value.
        value: u64,
        /// Why the value is invalid.
        reason: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unknown_policy_message_contains_name() {
        let e = ResourcePolicyError::UnknownPolicy {
            name: "ghost".into(),
        };
        assert!(e.to_string().contains("ghost"));
    }

    #[test]
    fn test_invalid_value_message_contains_field_and_value() {
        let e = ResourcePolicyError::InvalidValue {
            field: "timeout_ms".into(),
            value: 999_999_999,
            reason: "exceeds maximum".into(),
        };
        let msg = e.to_string();
        assert!(msg.contains("timeout_ms"));
        assert!(msg.contains("999999999"));
    }
}
