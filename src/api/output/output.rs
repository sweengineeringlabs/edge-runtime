//! `Output` — egress adapter contract.

use std::sync::Arc;
use swe_edge_egress_grpc::GrpcOutbound;
use swe_edge_egress_http::HttpOutbound;

/// Supplies the egress adapters the runtime uses for outbound calls.
pub trait Output: Send + Sync {
    fn http(&self) -> Arc<dyn HttpOutbound>;
    fn grpc(&self) -> Option<Arc<dyn GrpcOutbound>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_is_object_safe() {
        fn _assert(_: &dyn Output) {}
    }
}
