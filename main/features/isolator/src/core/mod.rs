//! Concrete isolation profile implementations.

pub(crate) mod noop;

#[cfg(all(target_os = "linux", feature = "seccomp"))]
pub(crate) mod seccomp;

use std::sync::Arc;

use swe_edge_egress_subprocess::{IsolationError, IsolationProfile};

use crate::api::profile_spec::ProfileSpec;

/// Resolve a [`ProfileSpec`] into a concrete [`IsolationProfile`] implementation.
pub(crate) fn resolve_profile(
    name: &str,
    spec: &ProfileSpec,
) -> Result<Arc<dyn IsolationProfile>, IsolationError> {
    match spec.kind.as_str() {
        "noop" => Ok(Arc::new(noop::NoopIsolator)),

        #[cfg(all(target_os = "linux", feature = "seccomp"))]
        "seccomp" => seccomp::SeccompIsolator::new(name, &spec.allowed_syscalls)
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
