//! Service factory for [`HttpIngress`].

pub use swe_edge_ingress_http::HttpIngress;

use crate::api::NoopHttpIngress;

impl NoopHttpIngress {
    /// Create a new [`NoopHttpIngress`] — a pass-through ingress handler for tests and
    /// composition roots that have not yet wired a real ingress implementation.
    pub fn create() -> Self {
        Self
    }
}
