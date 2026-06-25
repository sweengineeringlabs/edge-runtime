//! SAF surface for [`crate::api::CliRunner`] and [`crate::api::NoopCliRunner`].

pub(crate) use crate::api::NoopCliRunner;

impl NoopCliRunner {
    /// Create a no-op runner that always succeeds.
    pub fn create() -> Self {
        Self
    }
}
