//! `PolicySvc` — SAF factory implementations for resource policies and services.

use std::sync::Arc;

use swe_edge_configbuilder::ConfigSection as _;
use swe_edge_egress_subprocess::SubprocessRunner;

use crate::api::error::resource_policy_error::ResourcePolicyError;
use crate::api::types::policy::resource::resource_policy::ResourcePolicy;
use crate::api::types::policy::resource::resource_policy_config::ResourcePolicyConfig;
use crate::api::types::policy::resource::resource_policy_runner::ResourcePolicyRunner;
use crate::api::types::swe::noop_runtime_resource_policy::NoopRuntimeResourcePolicy;
use crate::api::types::swe::policy_svc::PolicySvc;
use crate::api::types::swe::swe_edge_runtime_resource_policy_factory::SweEdgeRuntimeResourcePolicyFactory;

impl PolicySvc {
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

    /// Wrap `inner` with `policy`, returning a [`ResourcePolicyRunner`].
    ///
    /// The runner injects policy limits into every [`SubprocessArgs`] before
    /// delegating to the inner runner.
    ///
    /// [`SubprocessArgs`]: swe_edge_egress_subprocess::SubprocessArgs
    pub fn create_policy_runner<R: SubprocessRunner>(
        inner: Arc<R>,
        policy: ResourcePolicy,
    ) -> ResourcePolicyRunner<R> {
        ResourcePolicyRunner { inner, policy }
    }

    /// Return a [`NoopRuntimeResourcePolicy`] implementing [`SweEdgeRuntimeResourcePolicy`].
    pub fn service() -> NoopRuntimeResourcePolicy {
        NoopRuntimeResourcePolicy
    }

    /// Return a [`NoopRuntimeResourcePolicy`] implementing [`Validator`].
    pub fn validator() -> NoopRuntimeResourcePolicy {
        NoopRuntimeResourcePolicy
    }
}

impl SweEdgeRuntimeResourcePolicyFactory {
    /// Create a [`NoopRuntimeResourcePolicy`] implementing [`SweEdgeRuntimeResourcePolicy`].
    pub fn create_swe_edge_runtime_resource_policy() -> NoopRuntimeResourcePolicy {
        NoopRuntimeResourcePolicy
    }

    /// Create a [`NoopRuntimeResourcePolicy`] implementing [`Validator`].
    pub fn create_validator() -> NoopRuntimeResourcePolicy {
        NoopRuntimeResourcePolicy
    }
}
