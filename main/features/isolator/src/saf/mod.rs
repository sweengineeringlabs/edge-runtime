//! SAF layer — public factory surface for isolation profiles.

use swe_edge_configbuilder::ConfigSection as _;
use swe_edge_egress_subprocess::IsolationProfile;

pub use crate::api::{IsolationProfileRegistry, IsolatorConfig, ProfileSpec};
use crate::core::noop::NoopIsolator;

/// Return a [`NoopIsolator`] — applies no OS-level restrictions.
///
/// Use in development, CI, or any environment where subprocess isolation
/// is not required.  For production, load profiles from TOML config via
/// [`create_profile_registry`].
pub fn create_noop_isolator() -> impl IsolationProfile {
    NoopIsolator
}

/// Load the subprocess isolation policy from `loader` and return a
/// populated [`IsolationProfileRegistry`].
///
/// # Errors
///
/// Returns [`IsolationError`] if the config cannot be loaded or any
/// profile spec is invalid.
///
/// [`IsolationError`]: swe_edge_egress_subprocess::IsolationError
pub fn create_profile_registry<L>(
    loader: &L,
) -> Result<IsolationProfileRegistry, swe_edge_egress_subprocess::IsolationError>
where
    L: swe_edge_configbuilder::Loader,
{
    let config = IsolatorConfig::load(loader).map_err(|e| {
        swe_edge_egress_subprocess::IsolationError::UnknownProfile {
            profile: format!("config load failed: {e}"),
        }
    })?;
    IsolationProfileRegistry::from_config(config)
}
