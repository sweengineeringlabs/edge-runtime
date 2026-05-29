//! `IsolationProfileRegistry` — named registry of compiled isolation profiles.

use std::collections::HashMap;
use std::sync::Arc;

use swe_edge_egress_subprocess::{IsolationError, IsolationProfile};

use crate::api::isolator_config::IsolatorConfig;
use crate::core::{noop::NoopIsolator, resolve_profile};

/// Named registry of [`IsolationProfile`] implementations.
///
/// Loaded once at daemon startup via [`IsolationProfileRegistry::from_config`].
/// Consumers resolve a profile by name and attach it to [`ProcessArgs`].
///
/// The `"noop"` profile is always registered regardless of config.
///
/// [`SubprocessArgs`]: swe_edge_egress_subprocess::SubprocessArgs
pub struct IsolationProfileRegistry {
    profiles: HashMap<String, Arc<dyn IsolationProfile>>,
}

impl IsolationProfileRegistry {
    /// Build a registry from a loaded [`IsolatorConfig`].
    ///
    /// Returns `Err` if any profile spec is invalid (e.g. unknown `kind`,
    /// or a seccomp profile with an unrecognised syscall name).
    pub fn from_config(config: IsolatorConfig) -> Result<Self, IsolationError> {
        let mut profiles: HashMap<String, Arc<dyn IsolationProfile>> = HashMap::new();

        // "noop" is always available.
        profiles.insert("noop".to_owned(), Arc::new(NoopIsolator));

        for (name, spec) in config.profiles {
            let profile = resolve_profile(&name, &spec)?;
            profiles.insert(name, profile);
        }

        Ok(Self { profiles })
    }

    /// Resolve a named profile.
    ///
    /// Returns [`IsolationError::UnknownProfile`] if the name is not registered.
    pub fn get(&self, name: &str) -> Result<Arc<dyn IsolationProfile>, IsolationError> {
        self.profiles
            .get(name)
            .cloned()
            .ok_or_else(|| IsolationError::UnknownProfile {
                profile: name.to_owned(),
            })
    }

    /// Returns the number of registered profiles.
    pub fn len(&self) -> usize {
        self.profiles.len()
    }

    /// Returns `true` if no profiles are registered (only possible if the
    /// built-in `"noop"` was somehow removed, which cannot happen via the
    /// public API).
    pub fn is_empty(&self) -> bool {
        self.profiles.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: from_config
    #[test]
    fn test_registry_from_default_config_has_noop() {
        let registry = IsolationProfileRegistry::from_config(IsolatorConfig::default()).unwrap();
        assert!(registry.get("noop").is_ok());
    }

    /// @covers: from_config, get
    #[test]
    fn test_registry_default_profile_is_noop() {
        let registry = IsolationProfileRegistry::from_config(IsolatorConfig::default()).unwrap();
        let profile = registry.get("default").unwrap();
        assert_eq!(profile.name(), "noop");
    }

    /// @covers: get
    #[test]
    fn test_registry_get_unknown_profile_returns_error() {
        let registry = IsolationProfileRegistry::from_config(IsolatorConfig::default()).unwrap();
        let err = registry.get("ghost").unwrap_err();
        assert!(matches!(err, IsolationError::UnknownProfile { .. }));
    }

    /// @covers: len
    #[test]
    fn test_registry_len_includes_builtin_noop() {
        let registry = IsolationProfileRegistry::from_config(IsolatorConfig::default()).unwrap();
        assert!(registry.len() >= 2); // "noop" + "default"
    }

    /// @covers: is_empty
    #[test]
    fn test_registry_is_not_empty_after_from_config() {
        let registry = IsolationProfileRegistry::from_config(IsolatorConfig::default()).unwrap();
        assert!(!registry.is_empty());
    }
}
