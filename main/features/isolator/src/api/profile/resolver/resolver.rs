//! `Resolver` — api contract for resolving profile specs into isolation profiles.

use std::sync::Arc;

use swe_edge_egress_subprocess::{IsolationError, IsolationProfile};

use crate::api::types::profile::profile_spec::ProfileSpec;

/// Contract for resolving a named [`ProfileSpec`] into a concrete [`IsolationProfile`].
///
/// The default implementation is `core::profile::resolver::ProfileResolver`.
pub trait Resolver {
    /// Resolve a [`ProfileSpec`] into a concrete [`IsolationProfile`] implementation.
    ///
    /// # Errors
    ///
    /// Returns [`IsolationError::UnknownProfile`] for unrecognised `kind` values,
    /// or [`IsolationError::UnsupportedPlatform`] for platform-specific profiles
    /// requested on an unsupported OS.
    fn resolve(name: &str, spec: &ProfileSpec)
        -> Result<Arc<dyn IsolationProfile>, IsolationError>;
}
