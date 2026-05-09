//! `Input` — ingress adapter contract.

use std::sync::Arc;
use swe_edge_ingress::{GrpcInbound, HttpInbound};

/// Supplies the ingress adapters the runtime binds traffic through.
pub trait Input: Send + Sync {
    fn http(&self) -> Option<Arc<dyn HttpInbound>>;
    fn grpc(&self) -> Option<Arc<dyn GrpcInbound>>;
    fn has_any(&self) -> bool { self.http().is_some() || self.grpc().is_some() }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: has_any
    #[test]
    fn test_has_any_returns_false_when_no_transports() {
        struct NoTransport;
        impl Input for NoTransport {
            fn http(&self) -> Option<Arc<dyn HttpInbound>> { None }
            fn grpc(&self) -> Option<Arc<dyn GrpcInbound>> { None }
        }
        assert!(!NoTransport.has_any());
    }
}
