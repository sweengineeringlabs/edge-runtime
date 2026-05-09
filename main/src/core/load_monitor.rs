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

use crate::api::load_monitor::{AutoscalePolicy, SharedCounters};

// ── HTTP wrapper ──────────────────────────────────────────────────────────────

/// Wraps an `HttpInbound` handler; records load metrics on every request.
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
        self.counters.on_start();
        let counters = Arc::clone(&self.counters);
        let fut = self.inner.handle(request, ctx);
        Box::pin(async move {
            let start = Instant::now();
            let result = fut.await;
            counters.on_end(start.elapsed().as_micros() as u64, result.is_err());
            result
        })
    }

    fn health_check(&self) -> BoxFuture<'_, HttpInboundResult<HttpHealthCheck>> {
        self.inner.health_check()
    }
}

// ── gRPC wrapper ──────────────────────────────────────────────────────────────

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

// ── Background sampler ────────────────────────────────────────────────────────

/// Ticks every second: pushes derived gauges into the provider and checks
/// autoscale thresholds.
pub(crate) struct BackgroundSampler {
    counters: SharedCounters,
    policy:   Option<AutoscalePolicy>,
}

impl BackgroundSampler {
    pub(crate) fn new(counters: SharedCounters, policy: Option<AutoscalePolicy>) -> Self {
        Self { counters, policy }
    }

    pub(crate) async fn run(self) {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
        loop {
            interval.tick().await;

            let active = self.counters.requests_in_flight.load(Ordering::Relaxed) as f64;
            let rps    = self.counters.requests_since_tick.swap(0, Ordering::Relaxed) as f64;
            let eps    = self.counters.errors_since_tick.swap(0, Ordering::Relaxed) as f64;
            let p99    = self.counters.latency_ring.lock().p99_ms();

            let p = &*self.counters.provider;
            p.record_gauge("edge_requests_active",      active, &[]);
            p.record_gauge("edge_requests_per_second",  rps,    &[]);
            p.record_gauge("edge_errors_per_second",    eps,    &[]);
            p.record_gauge("edge_request_latency_p99_ms", p99,  &[]);

            if let Some(ref policy) = self.policy {
                if active as u64 > policy.requests_active_max {
                    tracing::warn!(active, max = policy.requests_active_max,
                        "scale-out signal: requests_active exceeded threshold");
                }
                if rps as u64 > policy.requests_per_sec_max {
                    tracing::warn!(rps, max = policy.requests_per_sec_max,
                        "scale-out signal: requests_per_second exceeded threshold");
                }
                if p99 > policy.latency_p99_ms_max {
                    tracing::warn!(p99_ms = p99, max = policy.latency_p99_ms_max,
                        "scale-out signal: latency_p99_ms exceeded threshold");
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
    use swe_observ_metrics::create_local_metrics_backend;
    use crate::api::load_monitor::LoadCounters;

    fn counters() -> SharedCounters {
        Arc::new(LoadCounters::new(Arc::new(create_local_metrics_backend())))
    }

    /// @covers: HttpLoadMonitor::new
    #[test]
    fn test_http_load_monitor_new_does_not_panic() {
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

    /// @covers: HttpLoadMonitor::handle — records via provider
    #[tokio::test]
    async fn test_http_monitor_handle_records_request_via_provider() {
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
        m.handle(HttpRequest::get("/"), RequestContext::unauthenticated()).await.unwrap();
        assert_eq!(c.requests_in_flight.load(Ordering::Relaxed), 0);
        let snaps = c.provider.export();
        assert!(snaps.iter().any(|s| s.name == "edge_requests_total" && s.value == 1.0));
    }

    /// @covers: BackgroundSampler::new
    #[test]
    fn test_background_sampler_new_does_not_panic() {
        let _s = BackgroundSampler::new(counters(), None);
    }
}
