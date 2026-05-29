//! `IsolatorSvc` — SAF factory implementations for isolation profiles and services.

use std::collections::HashMap;
use std::sync::Arc;

use crate::api::traits::Validator;
use crate::api::traits::SweEdgeRuntimeIsolator;
use crate::api::types::profile::isolator_config::IsolatorConfig;
use crate::api::types::profile::isolation_profile_registry::IsolationProfileRegistry;
use crate::api::types::swe::swe_edge_runtime_isolator_factory::SweEdgeRuntimeIsolatorFactory;
use crate::api::types::swe::isolator_svc::IsolatorSvc;
use crate::core::noop::NoopIsolator;
use crate::core::profile_resolver::ProfileResolver;
use crate::core::DefaultSweEdgeRuntimeIsolator;

use swe_edge_configbuilder::ConfigSection as _;
use swe_edge_egress_subprocess::{IsolationError, IsolationProfile};

impl IsolatorSvc {
    /// Return a [`NoopIsolator`] — applies no OS-level restrictions.
    ///
    /// Use in development, CI, or any environment where subprocess isolation
    /// is not required.  For production, load profiles from TOML config via
    /// [`IsolatorSvc::create_profile_registry`].
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
    pub fn create_profile_registry(
        loader: &swe_edge_configbuilder::SectionLoaderImpl,
    ) -> Result<IsolationProfileRegistry, IsolationError> {
        let config = IsolatorConfig::load(loader).map_err(|e| {
            IsolationError::UnknownProfile {
                profile: format!("config load failed: {e}"),
            }
        })?;
        Self::build_registry(config)
    }

    /// Build a [`IsolationProfileRegistry`] directly from a pre-loaded [`IsolatorConfig`].
    ///
    /// Use when you already have a config in hand (e.g. in tests). For
    /// production, prefer [`IsolatorSvc::create_profile_registry`].
    ///
    /// # Errors
    ///
    /// Returns [`IsolationError`] if any profile spec is invalid.
    pub fn build_registry(
        config: IsolatorConfig,
    ) -> Result<IsolationProfileRegistry, IsolationError> {
        let mut profiles: HashMap<String, Arc<dyn IsolationProfile>> = HashMap::new();
        // "noop" is always available.
        profiles.insert("noop".to_owned(), Arc::new(NoopIsolator));
        for (name, spec) in config.profiles {
            let profile = ProfileResolver::resolve(&name, &spec)?;
            profiles.insert(name, profile);
        }
        Ok(IsolationProfileRegistry { profiles })
    }

    /// Return a default [`SweEdgeRuntimeIsolator`] implementation.
    pub fn service() -> impl SweEdgeRuntimeIsolator {
        DefaultSweEdgeRuntimeIsolator
    }

    /// Return a default [`Validator`] implementation.
    pub fn validator() -> impl Validator {
        DefaultSweEdgeRuntimeIsolator
    }
}

impl SweEdgeRuntimeIsolatorFactory {
    /// Create a default [`SweEdgeRuntimeIsolator`] implementation.
    pub fn create_swe_edge_runtime_isolator() -> impl SweEdgeRuntimeIsolator {
        DefaultSweEdgeRuntimeIsolator
    }

    /// Create a default [`Validator`] implementation.
    pub fn create_validator() -> impl Validator {
        DefaultSweEdgeRuntimeIsolator
    }
}
