//! `CompositeIngress` — composite ingress router contract.

use std::sync::Arc;
use swe_edge_ingress_grpc::GrpcIngress;

use crate::api::composite::types::composite_grpc_ingress::CompositeGrpcIngress;

/// Routes requests between a primary handler and a secondary (e.g. reflection).
pub trait CompositeIngress: Send + Sync {
    fn primary(&self) -> Arc<dyn GrpcIngress>;

    /// Construct a [`CompositeGrpcIngress`] from a primary and a reflection handler.
    fn new_composite(
        primary: Arc<dyn GrpcIngress>,
        reflection: Arc<dyn GrpcIngress>,
    ) -> CompositeGrpcIngress
    where
        Self: Sized,
    {
        CompositeGrpcIngress {
            primary,
            reflection,
        }
    }
}
