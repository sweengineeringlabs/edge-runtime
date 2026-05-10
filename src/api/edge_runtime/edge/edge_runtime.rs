//! `EdgeRuntime` — zero-size entry-point; use `EdgeRuntime::builder()`.

use super::edge_runtime_builder::EdgeRuntimeBuilder;
use swe_edge_ingress::GrpcInboundInterceptorChain;

/// Entry-point for the edge runtime.
pub struct EdgeRuntime;

impl EdgeRuntime {
    /// Create a new builder for assembling an edge runtime.
    pub fn builder() -> EdgeRuntimeBuilder {
        EdgeRuntimeBuilder {
            config:                    None,
            app_name:                  None,
            http_handler:              None,
            grpc_handler:              None,
            http_dispatcher:           None,
            grpc_dispatcher:           None,
            http_tls:                  None,
            grpc_tls:                  None,
            http_bearer_verifier:      None,
            grpc_interceptors:         GrpcInboundInterceptorChain::new(),
            grpc_allow_unauthenticated: false,
            egress_http:               None,
            egress_grpc:               None,
            lifecycle:                 None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: builder
    #[test]
    fn test_builder_starts_with_all_fields_none() {
        let b = EdgeRuntime::builder();
        assert!(b.config.is_none() && b.app_name.is_none() && b.egress_http.is_none());
    }
}
