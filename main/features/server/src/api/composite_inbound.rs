//! `CompositeInbound` — composite ingress router contract.

use std::sync::Arc;
use swe_edge_ingress::GrpcInbound;

/// Routes requests between a primary handler and a secondary (e.g. reflection).
pub trait CompositeInbound: Send + Sync {
    fn primary(&self) -> Arc<dyn GrpcInbound>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_composite_inbound_is_object_safe() {
        fn _accept(_: &dyn CompositeInbound) {}
    }
}
