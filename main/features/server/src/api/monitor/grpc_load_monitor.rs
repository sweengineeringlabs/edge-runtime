//! `GrpcLoadMonitor` — gRPC inbound load-monitoring wrapper interface.

use swe_edge_ingress::GrpcInbound;

/// Marker supertrait for gRPC inbound handlers that record load metrics.
pub trait GrpcLoadMonitor: GrpcInbound {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grpc_load_monitor_is_object_safe() {
        fn _assert(_: &dyn GrpcLoadMonitor) {}
    }
}
