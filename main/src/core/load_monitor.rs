//! Load monitor — HTTP/gRPC wrappers and background sampler.

use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::Instant;

use edge_domain::RequestContext;
use futures::future::BoxFuture;
use swe_edge_ingress::{
    GrpcHealthCheck, GrpcInbound, GrpcInboundResult, GrpcMessageStream,
    GrpcMetadata, GrpcRequest, GrpcResponse,
    HttpHealthCheck, HttpInbound, HttpInboundResult, HttpRequest, HttpResponse,
};

use crate::api::load_monitor::{AutoscalePolicy, LoadCounters, SharedCounters};

// ── HTTP wrapper ──────────────────────────────────────────────────────────────

/// Wraps an `HttpInbound` handler and records load metrics for every request.
pub(crate) struct HttpLoadMonitor {
    inner:    Arc<dyn HttpInbound>,
    counters: SharedCounters,
}

impl HttpLoadMonitor {
    pub(crate) fn new(inner: Arc<dyn HttpInbound>, counters: SharedCounters) -> Self {
        Self { inner, counters }
    }
}

impl HttpInbound for HttpLoadMonitor {
    fn handle(
        &self,
        request: HttpRequest,
        ctx:     RequestContext,
    ) -> BoxFuture<'_, HttpInboundResult<HttpResponse>> {
        self.counters.requests_active.fetch_add(1, Ordering::Relaxed);
        let counters = Arc::clone(&self.counters);
        let fut = self.inner.handle(request, ctx);
        Box::pin(async move {
            let start = Instant::now();
            let result = fut.await;
            let latency_us = start.elapsed().as_micros() as u64;
            counters.record(latency_us, result.is_err());
            result
        })
    }

    fn health_check(&self) -> BoxFuture<'_, HttpInboundResult<HttpHealthCheck>> {
        self.inner.health_check()
    }
}

// ── gRPC wrapper ──────────────────────────────────────────────────────────────

/// Wraps a `GrpcInbound` handler and records load metrics for every call.
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
        self.counters.requests_active.fetch_add(1, Ordering::Relaxed);
        let counters = Arc::clone(&self.counters);
        let fut = self.inner.handle_unary(request, ctx);
        Box::pin(async move {
            let start = Instant::now();
            let result = fut.await;
            let latency_us = start.elapsed().as_micros() as u64;
            counters.record(latency_us, result.is_err());
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
        self.counters.requests_active.fetch_add(1, Ordering::Relaxed);
        let counters = Arc::clone(&self.counters);
        let fut = self.inner.handle_stream(method, metadata, messages, ctx);
        Box::pin(async move {
            let start = Instant::now();
            let result = fut.await;
            let latency_us = start.elapsed().as_micros() as u64;
            counters.record(latency_us, result.is_err());
            result
        })
    }

    fn health_check(&self) -> BoxFuture<'_, GrpcInboundResult<GrpcHealthCheck>> {
        self.inner.health_check()
    }
}

// ── Background sampler ────────────────────────────────────────────────────────

/// Runs every second: computes derived metrics and checks autoscale thresholds.
pub(crate) struct BackgroundSampler {
    counters: SharedCounters,
    policy:   Option<AutoscalePolicy>,
}

impl BackgroundSampler {
    pub(crate) fn new(counters: SharedCounters, policy: Option<AutoscalePolicy>) -> Self {
        Self { counters, policy }
    }

    pub(crate) async fn run(self) {
        let mut prev_total  = 0u64;
        let mut prev_errors = 0u64;
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
        loop {
            interval.tick().await;

            let total  = self.counters.requests_total.load(Ordering::Relaxed);
            let errors = self.counters.errors_total.load(Ordering::Relaxed);
            let active = self.counters.requests_active.load(Ordering::Relaxed);

            let rps = total.saturating_sub(prev_total);
            let eps = errors.saturating_sub(prev_errors);
            let p99 = self.counters.latency_ring.lock().p99_ms();

            self.counters.snapshot_rps.store(rps, Ordering::Relaxed);
            self.counters.snapshot_p99_ms.store(p99, Ordering::Relaxed);
            self.counters.snapshot_err_per_sec.store(eps, Ordering::Relaxed);

            prev_total  = total;
            prev_errors = errors;

            if let Some(ref policy) = self.policy {
                if active > policy.requests_active_max {
                    tracing::warn!(
                        active, max = policy.requests_active_max,
                        "scale-out signal: requests_active exceeded threshold"
                    );
                }
                if rps > policy.requests_per_sec_max {
                    tracing::warn!(
                        rps, max = policy.requests_per_sec_max,
                        "scale-out signal: requests_per_second exceeded threshold"
                    );
                }
                if p99 > policy.latency_p99_ms_max {
                    tracing::warn!(
                        p99_ms = p99, max = policy.latency_p99_ms_max,
                        "scale-out signal: latency_p99_ms exceeded threshold"
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::Ordering;

    fn counters() -> SharedCounters { Arc::new(LoadCounters::new()) }

    /// @covers: HttpLoadMonitor::new
    #[test]
    fn test_http_load_monitor_new_does_not_panic() {
        use futures::future::BoxFuture;
        struct NullHttp;
        impl HttpInbound for NullHttp {
            fn handle(&self, _: HttpRequest, _: RequestContext) -> BoxFuture<'_, HttpInboundResult<HttpResponse>> {
                Box::pin(async { Ok(HttpResponse::new(200, vec![])) })
            }
            fn health_check(&self) -> BoxFuture<'_, HttpInboundResult<HttpHealthCheck>> {
                Box::pin(async { Ok(HttpHealthCheck::healthy()) })
            }
        }
        let _m = HttpLoadMonitor::new(Arc::new(NullHttp), counters());
    }

    /// @covers: GrpcLoadMonitor::new
    #[test]
    fn test_grpc_load_monitor_new_does_not_panic() {
        use swe_edge_ingress::GrpcInboundError;
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

    /// @covers: HttpLoadMonitor::handle — increments and decrements active
    #[tokio::test]
    async fn test_http_monitor_handle_updates_counters() {
        use futures::future::BoxFuture;
        struct OkHttp;
        impl HttpInbound for OkHttp {
            fn handle(&self, _: HttpRequest, _: RequestContext) -> BoxFuture<'_, HttpInboundResult<HttpResponse>> {
                Box::pin(async { Ok(HttpResponse::new(200, vec![])) })
            }
            fn health_check(&self) -> BoxFuture<'_, HttpInboundResult<HttpHealthCheck>> {
                Box::pin(async { Ok(HttpHealthCheck::healthy()) })
            }
        }
        let c = counters();
        let m = HttpLoadMonitor::new(Arc::new(OkHttp), Arc::clone(&c));
        let req = HttpRequest::get("/");
        m.handle(req, RequestContext::unauthenticated()).await.unwrap();
        assert_eq!(c.requests_active.load(Ordering::Relaxed), 0);
        assert_eq!(c.requests_total.load(Ordering::Relaxed), 1);
        assert_eq!(c.errors_total.load(Ordering::Relaxed), 0);
    }

    /// @covers: BackgroundSampler::new
    #[test]
    fn test_background_sampler_new_does_not_panic() {
        let _s = BackgroundSampler::new(counters(), None);
    }
}
