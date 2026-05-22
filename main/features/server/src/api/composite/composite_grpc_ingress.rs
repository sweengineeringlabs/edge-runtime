//! `CompositeGrpcIngress` — routes gRPC requests to primary or reflection handler.

use std::sync::Arc;
use swe_edge_ingress_grpc::GrpcIngress;

/// Holds the primary and reflection gRPC handlers for composite routing.
pub struct CompositeGrpcIngress {
    pub(crate) primary: Arc<dyn GrpcIngress>,
    pub(crate) reflection: Arc<dyn GrpcIngress>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use edge_domain::RequestContext;
    use futures::future::BoxFuture;
    use swe_edge_ingress_http::{
        GrpcHealthCheck, GrpcIngressError, GrpcIngressResult, GrpcMessageStream, GrpcMetadata,
        GrpcRequest, GrpcResponse,
    };

    struct StubGrpc;
    impl GrpcIngress for StubGrpc {
        fn handle_unary(
            &self,
            _: GrpcRequest,
            _: RequestContext,
        ) -> BoxFuture<'_, GrpcIngressResult<GrpcResponse>> {
            Box::pin(async { Err(GrpcIngressError::Unimplemented("stub".into())) })
        }
        fn handle_stream(
            &self,
            _: String,
            _: GrpcMetadata,
            _: GrpcMessageStream,
            _: RequestContext,
        ) -> BoxFuture<'_, GrpcIngressResult<(GrpcMessageStream, GrpcMetadata)>> {
            Box::pin(async { Err(GrpcIngressError::Unimplemented("stub".into())) })
        }
        fn health_check(&self) -> BoxFuture<'_, GrpcIngressResult<GrpcHealthCheck>> {
            Box::pin(async { Ok(GrpcHealthCheck::healthy()) })
        }
    }

    #[test]
    fn test_composite_grpc_ingress_holds_both_handlers() {
        let primary = Arc::new(StubGrpc) as Arc<dyn GrpcIngress>;
        let reflection = Arc::new(StubGrpc) as Arc<dyn GrpcIngress>;
        let c = CompositeGrpcIngress {
            primary: Arc::clone(&primary),
            reflection: Arc::clone(&reflection),
        };
        assert!(Arc::ptr_eq(&c.primary, &primary));
        assert!(Arc::ptr_eq(&c.reflection, &reflection));
    }
}
