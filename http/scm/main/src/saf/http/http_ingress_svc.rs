//! Service factory for [`HttpIngress`].

use crate::api::NoopHttpIngress;

impl NoopHttpIngress {
    /// Create a new [`NoopHttpIngress`] — a pass-through ingress handler for tests and
    /// composition roots that have not yet wired a real ingress implementation.
    pub fn create() -> Self {
        Self
    }
}
