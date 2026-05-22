//! `Egress` — egress adapter contract.

use std::sync::Arc;
use swe_edge_egress_grpc::GrpcEgress;
use swe_edge_egress_http::HttpEgress;

/// Supplies the egress adapters the runtime uses for outbound calls.
pub trait Egress: Send + Sync {
    /// Returns the HTTP outbound client.
    fn http(&self) -> Arc<dyn HttpEgress>;
    /// Returns the gRPC outbound client, if configured.
    fn grpc(&self) -> Option<Arc<dyn GrpcEgress>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_egress_is_object_safe() {
        fn _assert(_: &dyn Egress) {}
    }
}
