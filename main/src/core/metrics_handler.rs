//! Prometheus metrics HTTP handler — serves `GET /metrics`.

use std::sync::Arc;

use edge_domain::RequestContext;
use futures::future::BoxFuture;
use swe_edge_ingress::{
    HttpHealthCheck, HttpInbound, HttpInboundError, HttpInboundResult,
    HttpMethod, HttpRequest, HttpResponse,
};

use crate::api::load_monitor::SharedCounters;

/// Serves the Prometheus text exposition endpoint.
///
/// Bound to `[metrics] bind` (default `0.0.0.0:9090`) — never on the
/// primary `http_bind` so it is not exposed to public ingress.
pub(crate) struct MetricsHandler {
    counters: SharedCounters,
}

impl MetricsHandler {
    pub(crate) fn new(counters: SharedCounters) -> Self {
        Self { counters }
    }
}

impl HttpInbound for MetricsHandler {
    fn handle(
        &self,
        request: HttpRequest,
        _ctx:    RequestContext,
    ) -> BoxFuture<'_, HttpInboundResult<HttpResponse>> {
        let counters = Arc::clone(&self.counters);
        Box::pin(async move {
            if request.method != HttpMethod::Get {
                return Err(HttpInboundError::InvalidInput(
                    "metrics endpoint only accepts GET".into(),
                ));
            }
            let body = counters.snapshot().to_prometheus();
            let mut resp = HttpResponse::new(200, body.into_bytes());
            resp.headers.insert(
                "content-type".into(),
                "text/plain; version=0.0.4; charset=utf-8".into(),
            );
            Ok(resp)
        })
    }

    fn health_check(&self) -> BoxFuture<'_, HttpInboundResult<HttpHealthCheck>> {
        Box::pin(async { Ok(HttpHealthCheck::healthy()) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use crate::api::load_monitor::LoadCounters;

    fn handler() -> MetricsHandler {
        MetricsHandler::new(Arc::new(LoadCounters::new()))
    }

    /// @covers: MetricsHandler::handle — GET returns 200 with Prometheus body
    #[tokio::test]
    async fn test_handle_get_returns_prometheus_text() {
        let h    = handler();
        let req  = HttpRequest::get("/metrics");
        let resp = h.handle(req, RequestContext::unauthenticated()).await.unwrap();
        assert_eq!(resp.status, 200);
        let ct   = resp.header("content-type").map(str::to_owned);
        let body = String::from_utf8(resp.body).unwrap();
        assert!(body.contains("edge_requests_active"));
        assert!(body.contains("edge_requests_total"));
        assert_eq!(ct.as_deref(), Some("text/plain; version=0.0.4; charset=utf-8"));
    }

    /// @covers: MetricsHandler::handle — POST returns InvalidInput
    #[tokio::test]
    async fn test_handle_non_get_returns_invalid_input_error() {
        let h   = handler();
        let req = HttpRequest::post("/metrics");
        let err = h.handle(req, RequestContext::unauthenticated()).await.unwrap_err();
        assert!(matches!(err, HttpInboundError::InvalidInput(_)));
    }

    /// @covers: MetricsHandler::health_check
    #[tokio::test]
    async fn test_health_check_returns_healthy() {
        let h = handler();
        let hc = h.health_check().await.unwrap();
        assert!(hc.healthy);
    }
}
