//! `ResourcePolicyConfig` — TOML-loaded named resource policies.

use std::collections::HashMap;

use swe_edge_configbuilder::ConfigSection;

use crate::api::error::ResourcePolicyError;
use crate::api::policy::ResourcePolicy;

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
///
/// [resource_policies.batch]
/// name             = "batch"
/// timeout_ms       = 120000
/// output_bytes_cap = 4194304
/// cpu_time_ms      = 60000
/// memory_bytes     = 1073741824
/// ```
#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(transparent)]
pub struct ResourcePolicyConfig(pub HashMap<String, ResourcePolicy>);

impl ConfigSection for ResourcePolicyConfig {
    fn section_name() -> &'static str {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn config_with_default() -> ResourcePolicyConfig {
        let toml = r#"
            [default]
            name             = "default"
            timeout_ms       = 30000
            output_bytes_cap = 1048576
            cpu_time_ms      = 0
            memory_bytes     = 0
        "#;
        ResourcePolicyConfig(toml::from_str(toml).unwrap())
    }

    #[test]
    fn test_policy_config_section_name_is_resource_policies() {
        assert_eq!(ResourcePolicyConfig::section_name(), "resource_policies");
    }

    #[test]
    fn test_policy_config_get_existing_returns_policy() {
        let cfg = config_with_default();
        let policy = cfg.get("default").unwrap();
        assert_eq!(policy.timeout_ms, 30_000);
        assert_eq!(policy.output_bytes_cap, 1_048_576);
    }

    #[test]
    fn test_policy_config_get_unknown_returns_error() {
        let cfg = ResourcePolicyConfig::default();
        let err = cfg.get("ghost").unwrap_err();
        assert!(matches!(err, ResourcePolicyError::UnknownPolicy { .. }));
    }

    #[test]
    fn test_policy_config_default_is_empty() {
        assert!(ResourcePolicyConfig::default().is_empty());
    }
}
