//! `CompositeGrpcInbound` — routes gRPC requests to primary or reflection handler.

use std::sync::Arc;
use swe_edge_ingress::GrpcInbound;

/// Holds the primary and reflection gRPC handlers for composite routing.
pub(crate) struct CompositeGrpcInbound {
    pub(crate) primary:    Arc<dyn GrpcInbound>,
    pub(crate) reflection: Arc<dyn GrpcInbound>,
}
