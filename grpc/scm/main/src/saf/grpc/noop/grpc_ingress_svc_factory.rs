//! SAF factory surface — NoopGrpcIngress inherent methods.

use std::sync::Arc;

use crate::api::NoopGrpcIngress;

impl NoopGrpcIngress {
    /// Wrap a new `NoopGrpcIngress` in an `Arc` for use as a [`GrpcIngress`] trait object.
    pub fn create() -> Arc<Self> {
        Arc::new(Self)
    }
}
