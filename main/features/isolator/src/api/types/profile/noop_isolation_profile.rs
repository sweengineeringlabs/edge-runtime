//! `NoopIsolationProfile` — concrete no-op isolation profile for dev and CI.

use swe_edge_egress_subprocess::{IsolationError, IsolationProfile};

/// Profile name constant, shared with the gateway registry entry.
const PROFILE_NAME: &str = "noop";

/// Concrete no-op isolation profile.
///
/// Applies no OS-level restrictions to the subprocess.  Always succeeds.
/// Returned by [`IsolatorSvc::create_noop_isolator`] for use in development,
/// CI, or any environment where subprocess isolation is not required.
///
/// Never use in production for untrusted or arbitrary code.
#[derive(Debug, Default)]
pub struct NoopIsolationProfile;

impl IsolationProfile for NoopIsolationProfile {
    fn name(&self) -> &str {
        PROFILE_NAME
    }

    fn configure(&self, _cmd: &mut tokio::process::Command) -> Result<(), IsolationError> {
        Ok(())
    }

    fn apply(&self, _child: &mut tokio::process::Child) -> Result<(), IsolationError> {
        Ok(())
    }
}
