//! `CompositeGrpcInbound` — routes gRPC requests to primary or reflection handler.

use std::sync::Arc;
use swe_edge_ingress::GrpcInbound;

/// Holds the primary and reflection gRPC handlers for composite routing.
pub struct CompositeGrpcInbound {
    pub(crate) primary:    Arc<dyn GrpcInbound>,
    pub(crate) reflection: Arc<dyn GrpcInbound>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_domain::RequestContext;
    use futures::future::BoxFuture;
    use swe_edge_ingress::{
        GrpcHealthCheck, GrpcInboundError, GrpcInboundResult, GrpcMessageStream,
        GrpcMetadata, GrpcRequest, GrpcResponse,
    };

    struct StubGrpc;
    impl GrpcInbound for StubGrpc {
        fn handle_unary(&self, _: GrpcRequest, _: RequestContext) -> BoxFuture<'_, GrpcInboundResult<GrpcResponse>>
        { Box::pin(async { Err(GrpcInboundError::Unimplemented("stub".into())) }) }
        fn handle_stream(&self, _: String, _: GrpcMetadata, _: GrpcMessageStream, _: RequestContext) -> BoxFuture<'_, GrpcInboundResult<(GrpcMessageStream, GrpcMetadata)>>
        { Box::pin(async { Err(GrpcInboundError::Unimplemented("stub".into())) }) }
        fn health_check(&self) -> BoxFuture<'_, GrpcInboundResult<GrpcHealthCheck>>
        { Box::pin(async { Ok(GrpcHealthCheck::healthy()) }) }
    }

    #[test]
    fn test_composite_grpc_inbound_holds_both_handlers() {
        let primary    = Arc::new(StubGrpc) as Arc<dyn GrpcInbound>;
        let reflection = Arc::new(StubGrpc) as Arc<dyn GrpcInbound>;
        let c = CompositeGrpcInbound { primary: Arc::clone(&primary), reflection: Arc::clone(&reflection) };
        assert!(Arc::ptr_eq(&c.primary, &primary));
        assert!(Arc::ptr_eq(&c.reflection, &reflection));
    }
}
