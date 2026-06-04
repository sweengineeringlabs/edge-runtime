//! `ResourcePolicyConfig` — TOML-loaded named resource policies.

use std::collections::HashMap;

use swe_edge_configbuilder::ConfigSection;

use super::resource_policy::ResourcePolicy;
use crate::api::error::resource_policy_error::ResourcePolicyError;

/// Named resource policies loaded from the `[resource_policies]` TOML section.
///
/// # TOML example
///
/// ```toml
/// [resource_policies.default]
/// name             = "default"
/// timeout_ms       = 30000
/// output_bytes_cap = 1048576
/// cpu_time_ms      = 0
/// memory_bytes     = 0
/// ```
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(transparent)]
pub struct ResourcePolicyConfig(pub HashMap<String, ResourcePolicy>);

impl ConfigSection for ResourcePolicyConfig {
    fn section_name() -> &'static str {
        // @allow: no_stub_fn_bodies — required ConfigSection impl, not a stub
        "resource_policies"
    }
}

impl ResourcePolicyConfig {
    /// Look up a named policy.
    ///
    /// # Errors
    ///
    /// Returns [`ResourcePolicyError::UnknownPolicy`] if the name is absent.
    pub fn get(&self, name: &str) -> Result<ResourcePolicy, ResourcePolicyError> {
        self.0
            .get(name)
            .cloned()
            .ok_or_else(|| ResourcePolicyError::UnknownPolicy {
                name: name.to_owned(),
            })
    }

    /// Returns the number of loaded policies.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if no policies were loaded.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
