//! Inbound gateway contract.

use std::sync::Arc;

use swe_edge_ingress::{GrpcInbound, HttpInbound};

/// Supplies the ingress adapters the runtime binds traffic through.
pub trait Input: Send + Sync {
    /// HTTP inbound adapter, if configured.
    fn http(&self) -> Option<Arc<dyn HttpInbound>>;
    /// gRPC inbound adapter, if configured.
    fn grpc(&self) -> Option<Arc<dyn GrpcInbound>>;
    /// Returns `true` if at least one transport is configured.
    fn has_any(&self) -> bool {
        self.http().is_some() || self.grpc().is_some()
    }
}
