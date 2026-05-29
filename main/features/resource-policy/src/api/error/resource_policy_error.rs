//! `ResourcePolicyError` — errors from policy loading and resolution.

/// Errors from loading or resolving a [`ResourcePolicy`].
///
/// [`ResourcePolicy`]: crate::ResourcePolicy
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
