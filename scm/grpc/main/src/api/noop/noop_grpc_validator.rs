//! [`NoopGrpcValidator`] — no-op validator stub declared in `api/noop/` per SEA noop-exemption.

/// A no-op [`crate::api::Validator`] that always returns `Ok(())`.
///
/// Suitable for tests and composition roots where no validation policy is required.
pub struct NoopGrpcValidator;
