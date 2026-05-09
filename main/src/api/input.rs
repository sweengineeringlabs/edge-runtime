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

/// Default [`Input`] implementation — holds optional ingress adapters by `Arc`.
///
/// At least one transport must be configured before
/// [`RuntimeManager::start`] is called.
pub struct DefaultInput {
    pub(crate) http: Option<Arc<dyn HttpInbound>>,
    pub(crate) grpc: Option<Arc<dyn GrpcInbound>>,
}

impl DefaultInput {
    /// Start with HTTP as the sole transport.
    pub fn new_http(http: Arc<dyn HttpInbound>) -> Self {
        Self { http: Some(http), grpc: None }
    }
    /// Start with gRPC as the sole transport.
    pub fn new_grpc(grpc: Arc<dyn GrpcInbound>) -> Self {
        Self { http: None, grpc: Some(grpc) }
    }
    /// Construct with no transports configured.
    pub fn empty() -> Self { Self { http: None, grpc: None } }
    /// Add (or replace) the HTTP transport.
    pub fn with_http(mut self, http: Arc<dyn HttpInbound>) -> Self {
        self.http = Some(http); self
    }
    /// Add (or replace) the gRPC transport.
    pub fn with_grpc(mut self, grpc: Arc<dyn GrpcInbound>) -> Self {
        self.grpc = Some(grpc); self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: Input::has_any
    #[test]
    fn test_has_any_false_with_no_transports() {
        struct NoTransport;
        impl Input for NoTransport {
            fn http(&self) -> Option<Arc<dyn HttpInbound>> { None }
            fn grpc(&self) -> Option<Arc<dyn GrpcInbound>> { None }
        }
        assert!(!NoTransport.has_any());
    }

    /// @covers: DefaultInput struct declaration
    #[test]
    fn test_default_input_fields_accessible_within_crate() {
        let input = DefaultInput { http: None, grpc: None };
        assert!(input.http.is_none());
        assert!(input.grpc.is_none());
    }
}
