//! SAF layer — public factory surface for resource policy.

use std::sync::Arc;

use swe_edge_configbuilder::ConfigSection as _;
use swe_edge_egress_subprocess::SubprocessRunner;

pub use crate::api::{ResourceLimits, ResourcePolicy, ResourcePolicyConfig, ResourcePolicyError};
pub use crate::core::policy_runner::ResourcePolicyRunner;
pub use crate::core::resolver::ResourceLimitsResolver;

/// Wrap `inner` with `policy`, returning a [`ResourcePolicyRunner`].
///
/// The runner injects policy limits into every [`SubprocessArgs`] before
/// delegating to the inner runner.
///
/// [`SubprocessArgs`]: swe_edge_egress_subprocess::SubprocessArgs
pub fn create_resource_policy_runner<R: SubprocessRunner>(
    inner: Arc<R>,
    policy: ResourcePolicy,
) -> ResourcePolicyRunner<R> {
    ResourcePolicyRunner::new(inner, policy)
}

/// Load the resource policy config from `loader` and return the named policy.
///
/// # Errors
///
/// Returns [`ResourcePolicyError`] if config cannot be loaded or the named
/// policy is absent.
pub fn load_policy(
    loader: &swe_edge_configbuilder::SectionLoaderImpl,
    name: &str,
) -> Result<ResourcePolicy, ResourcePolicyError> {
    let config =
        ResourcePolicyConfig::load(loader).map_err(|e| ResourcePolicyError::ConfigParse {
            reason: e.to_string(),
        })?;
    config.get(name)
}
