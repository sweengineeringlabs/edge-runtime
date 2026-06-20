//! [`NoopValidator`] — no-op validator stub declared in `api/` per SEA `pub_types_in_api_only`.

/// A no-op [`crate::api::Validator`] that always returns `Ok(())`.
///
/// Suitable for tests and composition roots where no validation policy is required.
pub struct NoopValidator;
