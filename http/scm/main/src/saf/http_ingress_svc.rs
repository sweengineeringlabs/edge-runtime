//! Service factory for [`HttpIngress`] — SAF re-export surface.

pub use crate::api::{
    FormPart, HttpAuth, HttpBody, HttpDecodeFn, HttpEncodeFn, HttpHealthCheck, HttpIngress,
    HttpIngressError, HttpIngressResult, HttpMethod, HttpRequest, HttpRequestBuilder, HttpResponse,
    NoopHttpIngress,
};

impl NoopHttpIngress {
    /// Create a new [`NoopHttpIngress`] — a pass-through ingress handler for tests and
    /// composition roots that have not yet wired a real ingress implementation.
    pub fn create() -> Self {
        Self
    }
}
