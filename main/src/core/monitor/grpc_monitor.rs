use std::sync::Arc;
use std::time::Instant;

use edge_domain::RequestContext;
use futures::future::BoxFuture;
use swe_edge_ingress::{
    GrpcHealthCheck, GrpcInbound, GrpcInboundResult, GrpcMessageStream,
    GrpcMetadata, GrpcRequest, GrpcResponse,
};

use crate::api::monitor::SharedCounters;

/// Wraps a `GrpcInbound` handler; records load metrics on every call.
pub(crate) struct GrpcLoadMonitor {
    inner:    Arc<dyn GrpcInbound>,
    counters: SharedCounters,
}

impl GrpcLoadMonitor {
    pub(crate) fn new(inner: Arc<dyn GrpcInbound>, counters: SharedCounters) -> Self {
        Self { inner, counters }
    }
}

impl GrpcInbound for GrpcLoadMonitor {
    fn handle_unary(
        &self,
        request: GrpcRequest,
        ctx:     RequestContext,
    ) -> BoxFuture<'_, GrpcInboundResult<GrpcResponse>> {
        self.counters.on_start();
        let counters = Arc::clone(&self.counters);
        let fut = self.inner.handle_unary(request, ctx);
        Box::pin(async move {
            let start = Instant::now();
            let result = fut.await;
            counters.on_end(start.elapsed().as_micros() as u64, result.is_err());
            result
        })
    }

    fn handle_stream(
        &self,
        method:   String,
        metadata: GrpcMetadata,
        messages: GrpcMessageStream,
        ctx:      RequestContext,
    ) -> BoxFuture<'_, GrpcInboundResult<(GrpcMessageStream, GrpcMetadata)>> {
        self.counters.on_start();
        let counters = Arc::clone(&self.counters);
        let fut = self.inner.handle_stream(method, metadata, messages, ctx);
        Box::pin(async move {
            let start = Instant::now();
            let result = fut.await;
            counters.on_end(start.elapsed().as_micros() as u64, result.is_err());
            result
        })
    }

    fn health_check(&self) -> BoxFuture<'_, GrpcInboundResult<GrpcHealthCheck>> {
        self.inner.health_check()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use swe_observ_metrics::create_local_metrics_backend;
    use swe_edge_ingress::GrpcInboundError;
    use crate::api::monitor::LoadCounters;

    fn counters() -> SharedCounters {
        Arc::new(LoadCounters::new(Arc::new(create_local_metrics_backend())))
    }

    /// @covers: GrpcLoadMonitor::new
    #[test]
    fn test_grpc_load_monitor_new_does_not_panic() {
        struct NullGrpc;
        impl GrpcInbound for NullGrpc {
            fn handle_unary(&self, _: GrpcRequest, _: RequestContext) -> BoxFuture<'_, GrpcInboundResult<GrpcResponse>> {
                Box::pin(async { Err(GrpcInboundError::Unimplemented("stub".into())) })
            }
            fn handle_stream(&self, _: String, _: GrpcMetadata, _: GrpcMessageStream, _: RequestContext) -> BoxFuture<'_, GrpcInboundResult<(GrpcMessageStream, GrpcMetadata)>> {
                Box::pin(async { Err(GrpcInboundError::Unimplemented("stub".into())) })
            }
            fn health_check(&self) -> BoxFuture<'_, GrpcInboundResult<GrpcHealthCheck>> {
                Box::pin(async { Ok(GrpcHealthCheck::healthy()) })
            }
        }
        let _m = GrpcLoadMonitor::new(Arc::new(NullGrpc), counters());
    }
}
