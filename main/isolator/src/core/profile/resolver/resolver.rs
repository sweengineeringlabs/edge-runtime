//! `ProfileResolver` — resolves a [`ProfileSpec`] into a concrete [`IsolationProfile`].

use std::sync::Arc;

use swe_edge_egress_subprocess::{IsolationError, IsolationProfile};

use crate::api::types::profile::profile_spec::ProfileSpec;

/// Resolves a named [`ProfileSpec`] into the corresponding [`IsolationProfile`] implementation.
pub(crate) struct ProfileResolver;

impl ProfileResolver {
    /// Resolve a [`ProfileSpec`] into a concrete [`IsolationProfile`] implementation.
    ///
    /// # Errors
    ///
    /// Returns [`IsolationError::UnknownProfile`] for unrecognised `kind` values,
    /// or [`IsolationError::UnsupportedPlatform`] when a platform-specific profile
    /// is requested on an unsupported OS.
    pub(crate) fn resolve(
        name: &str,
        spec: &ProfileSpec,
    ) -> Result<Arc<dyn IsolationProfile>, IsolationError> {
        match spec.kind.as_str() {
            "noop" => Ok(Arc::new(crate::core::noop::NoopIsolator)),

            #[cfg(all(target_os = "linux", feature = "seccomp"))]
            "seccomp" => crate::core::seccomp::SeccompIsolator::new(name, &spec.allowed_syscalls)
                .map(|s| Arc::new(s) as Arc<dyn IsolationProfile>),

            #[cfg(not(all(target_os = "linux", feature = "seccomp")))]
            "seccomp" => Err(IsolationError::UnsupportedPlatform {
                profile: name.to_owned(),
            }),

            "job_object" => Err(IsolationError::UnsupportedPlatform {
                profile: name.to_owned(),
            }),

            other => Err(IsolationError::UnknownProfile {
                profile: other.to_owned(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::types::profile::profile_spec::ProfileSpec;

    fn noop_spec() -> ProfileSpec {
        ProfileSpec {
            kind: "noop".to_owned(),
            allowed_syscalls: Vec::new(),
            cpu_rate_hundredths: 0,
            memory_limit_bytes: 0,
            kill_on_job_close: true,
        }
    }

    /// @covers: resolve
    #[test]
    fn test_profile_resolver_resolve_noop_returns_ok() {
        let result = ProfileResolver::resolve("noop", &noop_spec());
        assert!(result.is_ok());
    }

    /// @covers: resolve
    #[test]
    fn test_profile_resolver_resolve_unknown_returns_error() {
        let spec = ProfileSpec {
            kind: "unknown_kind".to_owned(),
            ..noop_spec()
        };
        let err = ProfileResolver::resolve("test", &spec).unwrap_err();
        assert!(matches!(err, IsolationError::UnknownProfile { .. }));
    }

    /// @covers: resolve
    #[test]
    fn test_profile_resolver_resolve_job_object_returns_unsupported() {
        let spec = ProfileSpec {
            kind: "job_object".to_owned(),
            ..noop_spec()
        };
        let err = ProfileResolver::resolve("test", &spec).unwrap_err();
        assert!(matches!(err, IsolationError::UnsupportedPlatform { .. }));
    }
}
