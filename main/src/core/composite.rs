//! `CompositeGrpcInbound` — routes reflection methods to `ReflectionService`,
//! all other methods to the primary dispatcher.

use std::sync::Arc;

use edge_domain::RequestContext;
use futures::future::BoxFuture;
use swe_edge_ingress::{
    GrpcHealthCheck, GrpcInbound, GrpcInboundResult, GrpcMessageStream,
    GrpcMetadata, GrpcRequest, GrpcResponse,
};

const REFLECTION_PREFIX: &str = "/grpc.reflection.";

pub(crate) struct CompositeGrpcInbound {
    primary:    Arc<dyn GrpcInbound>,
    reflection: Arc<dyn GrpcInbound>,
}

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
