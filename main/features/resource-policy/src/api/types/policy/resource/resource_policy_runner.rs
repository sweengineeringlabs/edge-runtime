//! `ResourcePolicyRunner` — `SubprocessRunner` decorator that injects resource policy.

use std::sync::Arc;

use swe_edge_egress_subprocess::{BoxFuture, SubprocessArgs, SubprocessResult, SubprocessRunner};

use super::resource_policy::ResourcePolicy;

/// A [`SubprocessRunner`] decorator that injects [`ResourcePolicy`] limits into
/// every [`SubprocessArgs`] before delegating to the inner runner.
///
/// Fields already set by the caller in `SubprocessArgs` are preserved —
/// `ResourcePolicyRunner` only fills `None` slots.
///
/// [`SubprocessArgs`]: swe_edge_egress_subprocess::SubprocessArgs
pub struct ResourcePolicyRunner<R: SubprocessRunner> {
    pub(crate) inner: Arc<R>,
    pub(crate) policy: ResourcePolicy,
}

impl<R: SubprocessRunner> ResourcePolicyRunner<R> {
    /// Return a reference to the active policy.
    pub fn policy(&self) -> &ResourcePolicy {
        &self.policy
    }
}

impl<R: SubprocessRunner> SubprocessRunner for ResourcePolicyRunner<R> {
    fn run(&self, mut args: SubprocessArgs) -> BoxFuture<'_, SubprocessResult> {
        self.policy.inject_into(&mut args);
        self.inner.run(args)
    }
}
