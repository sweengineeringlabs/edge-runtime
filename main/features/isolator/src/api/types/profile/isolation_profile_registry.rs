//! `IsolationProfileRegistry` — named registry of compiled isolation profiles.

use std::collections::HashMap;
use std::sync::Arc;

use swe_edge_egress_subprocess::{IsolationError, IsolationProfile};

/// Named registry of [`IsolationProfile`] implementations.
///
/// Loaded once at daemon startup via [`IsolatorSvc::build_registry`].
/// Consumers resolve a profile by name and attach it to [`SubprocessArgs`].
///
/// The `"noop"` profile is always registered regardless of config.
///
/// [`IsolatorSvc::build_registry`]: crate::IsolatorSvc::build_registry
/// [`SubprocessArgs`]: swe_edge_egress_subprocess::SubprocessArgs
pub struct IsolationProfileRegistry {
    pub(crate) profiles: HashMap<String, Arc<dyn IsolationProfile>>,
}

impl std::fmt::Debug for IsolationProfileRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let names: Vec<&str> = self.profiles.keys().map(String::as_str).collect();
        f.debug_struct("IsolationProfileRegistry")
            .field("profiles", &names)
            .finish()
    }
}

impl IsolationProfileRegistry {
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

    /// Returns `true` if no profiles are registered.
    pub fn is_empty(&self) -> bool {
        self.profiles.is_empty()
    }
}
