use std::sync::Arc;
use std::time::Instant;

use edge_domain::RequestContext;
use futures::future::BoxFuture;
use swe_edge_ingress::{
    HttpHealthCheck, HttpInbound, HttpInboundResult, HttpRequest, HttpResponse,
};

use crate::api::monitor::SharedCounters;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::Ordering;
    use swe_observ_metrics::create_local_metrics_backend;
    use crate::api::monitor::LoadCounters;

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
}
