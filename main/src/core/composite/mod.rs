//! `CompositeGrpcInbound` — implementation.

use std::sync::Arc;

use edge_domain::RequestContext;
use futures::future::BoxFuture;
use swe_edge_ingress::{
    GrpcHealthCheck, GrpcInbound, GrpcInboundResult, GrpcMessageStream,
    GrpcMetadata, GrpcRequest, GrpcResponse,
};

pub(crate) use crate::api::composite::CompositeGrpcInbound;

const REFLECTION_PREFIX: &str = "/grpc.reflection.";

impl CompositeGrpcInbound {
    pub(crate) fn new(
        primary:    Arc<dyn GrpcInbound>,
        reflection: Arc<dyn GrpcInbound>,
    ) -> Self {
        Self { primary, reflection }
    }

    fn route(&self, method: &str) -> Arc<dyn GrpcInbound> {
        if method.starts_with(REFLECTION_PREFIX) {
            Arc::clone(&self.reflection)
        } else {
            Arc::clone(&self.primary)
        }
    }
}

impl GrpcInbound for CompositeGrpcInbound {
    fn handle_unary(
        &self,
        request: GrpcRequest,
        ctx:     RequestContext,
    ) -> BoxFuture<'_, GrpcInboundResult<GrpcResponse>> {
        let handler = self.route(&request.method);
        Box::pin(async move { handler.handle_unary(request, ctx).await })
    }

    fn handle_stream(
        &self,
        method:   String,
        metadata: GrpcMetadata,
        messages: GrpcMessageStream,
        ctx:      RequestContext,
    ) -> BoxFuture<'_, GrpcInboundResult<(GrpcMessageStream, GrpcMetadata)>> {
        let handler = self.route(&method);
        Box::pin(async move { handler.handle_stream(method, metadata, messages, ctx).await })
    }

    fn health_check(&self) -> BoxFuture<'_, GrpcInboundResult<GrpcHealthCheck>> {
        self.primary.health_check()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use futures::future::BoxFuture;
    use parking_lot::Mutex;
    use swe_edge_ingress::{GrpcInboundError, GrpcMetadata, GrpcRequest};

    #[derive(Default)]
    struct TrackingHandler { called: Mutex<bool> }

    impl TrackingHandler {
        fn was_called(&self) -> bool { *self.called.lock() }
    }

    impl GrpcInbound for TrackingHandler {
        fn handle_unary(&self, _req: GrpcRequest, _ctx: RequestContext) -> BoxFuture<'_, GrpcInboundResult<GrpcResponse>> {
            *self.called.lock() = true;
            Box::pin(async { Err(GrpcInboundError::Unimplemented("stub".into())) })
        }
        fn handle_stream(&self, _m: String, _md: GrpcMetadata, _ms: GrpcMessageStream, _ctx: RequestContext) -> BoxFuture<'_, GrpcInboundResult<(GrpcMessageStream, GrpcMetadata)>> {
            *self.called.lock() = true;
            Box::pin(async { Err(GrpcInboundError::Unimplemented("stub".into())) })
        }
        fn health_check(&self) -> BoxFuture<'_, GrpcInboundResult<GrpcHealthCheck>> {
            Box::pin(async { Ok(GrpcHealthCheck::healthy()) })
        }
    }

    fn req(method: &str) -> GrpcRequest {
        GrpcRequest::new(method, vec![], std::time::Duration::from_secs(5))
    }

    /// @covers: CompositeGrpcInbound::new
    #[test]
    fn test_new_creates_composite_with_both_handlers() {
        let primary    = Arc::new(TrackingHandler::default());
        let reflection = Arc::new(TrackingHandler::default());
        let _composite = CompositeGrpcInbound::new(Arc::clone(&primary) as Arc<dyn GrpcInbound>, Arc::clone(&reflection) as Arc<dyn GrpcInbound>);
        assert!(!primary.was_called());
        assert!(!reflection.was_called());
    }

    /// @covers: CompositeGrpcInbound::route — reflection prefix routes to reflection
    #[tokio::test]
    async fn test_handle_unary_routes_reflection_path_to_reflection_handler() {
        let primary    = Arc::new(TrackingHandler::default());
        let reflection = Arc::new(TrackingHandler::default());
        let composite  = CompositeGrpcInbound::new(
            Arc::clone(&primary)    as Arc<dyn GrpcInbound>,
            Arc::clone(&reflection) as Arc<dyn GrpcInbound>,
        );
        let _ = composite.handle_unary(req("/grpc.reflection.v1alpha.ServerReflection/ServerReflectionInfo"), RequestContext::unauthenticated()).await;
        assert!(!primary.was_called(), "primary must not be called for reflection path");
        assert!(reflection.was_called(), "reflection must be called for reflection path");
    }

    /// @covers: CompositeGrpcInbound::route — non-reflection path routes to primary
    #[tokio::test]
    async fn test_handle_unary_routes_non_reflection_path_to_primary_handler() {
        let primary    = Arc::new(TrackingHandler::default());
        let reflection = Arc::new(TrackingHandler::default());
        let composite  = CompositeGrpcInbound::new(
            Arc::clone(&primary)    as Arc<dyn GrpcInbound>,
            Arc::clone(&reflection) as Arc<dyn GrpcInbound>,
        );
        let _ = composite.handle_unary(req("/my.Service/Method"), RequestContext::unauthenticated()).await;
        assert!(primary.was_called(), "primary must be called for non-reflection path");
        assert!(!reflection.was_called(), "reflection must not be called for non-reflection path");
    }
}
