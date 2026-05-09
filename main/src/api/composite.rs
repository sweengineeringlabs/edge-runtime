//! `CompositeGrpcInbound` — type declaration.
//!
//! Routes inbound gRPC calls to either the primary dispatcher or the
//! reflection service based on the method path prefix.

use std::sync::Arc;

use swe_edge_ingress::GrpcInbound;

/// A `GrpcInbound` that routes `/grpc.reflection.*` calls to a reflection
/// handler and all other calls to the primary handler.
pub(crate) struct CompositeGrpcInbound {
    pub(crate) primary:    Arc<dyn GrpcInbound>,
    pub(crate) reflection: Arc<dyn GrpcInbound>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use futures::future::BoxFuture;
    use edge_domain::RequestContext;
    use swe_edge_ingress::{
        GrpcHealthCheck, GrpcInboundError, GrpcInboundResult, GrpcMessageStream,
        GrpcMetadata, GrpcRequest, GrpcResponse,
    };

    struct NullHandler;
    impl GrpcInbound for NullHandler {
        fn handle_unary(&self, _: GrpcRequest, _: RequestContext) -> BoxFuture<'_, GrpcInboundResult<GrpcResponse>> {
            Box::pin(async { Err(GrpcInboundError::Unimplemented("null".into())) })
        }
        fn handle_stream(&self, _: String, _: GrpcMetadata, _: GrpcMessageStream, _: RequestContext) -> BoxFuture<'_, GrpcInboundResult<(GrpcMessageStream, GrpcMetadata)>> {
            Box::pin(async { Err(GrpcInboundError::Unimplemented("null".into())) })
        }
        fn health_check(&self) -> BoxFuture<'_, GrpcInboundResult<GrpcHealthCheck>> {
            Box::pin(async { Ok(GrpcHealthCheck::healthy()) })
        }
    }

    /// @covers: CompositeGrpcInbound — struct is constructible
    #[test]
    fn test_composite_struct_is_constructible() {
        let primary    = Arc::new(NullHandler) as Arc<dyn GrpcInbound>;
        let reflection = Arc::new(NullHandler) as Arc<dyn GrpcInbound>;
        let _ = CompositeGrpcInbound { primary, reflection };
    }
}
