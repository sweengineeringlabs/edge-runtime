//! Outbound gateway contract.

use std::sync::Arc;

use swe_edge_egress_grpc::GrpcOutbound;
use swe_edge_egress_http::HttpOutbound;

/// Supplies the egress adapters the runtime uses for outbound calls.
pub trait Output: Send + Sync {
    /// HTTP outbound adapter (required).
    fn http(&self) -> Arc<dyn HttpOutbound>;
    /// gRPC outbound adapter, if configured.
    fn grpc(&self) -> Option<Arc<dyn GrpcOutbound>>;
}
