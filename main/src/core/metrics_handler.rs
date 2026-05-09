//! Prometheus metrics HTTP handler — serves `GET /metrics`.
//!
//! Calls `MetricsProvider::export()` and formats the resulting
//! `Vec<MetricSnapshot>` as Prometheus text exposition (version 0.0.4).

use std::sync::Arc;

use edge_domain::RequestContext;
use futures::future::BoxFuture;
use swe_edge_ingress::{
    HttpHealthCheck, HttpInbound, HttpInboundError, HttpInboundResult,
    HttpMethod, HttpRequest, HttpResponse,
};
use swe_observ_metrics::{MetricType, MetricsProvider};

use crate::api::load_monitor::SharedCounters;

/// Serves the Prometheus text exposition endpoint.
///
/// Bound to `[metrics] bind` (default `0.0.0.0:9090`) — never on
/// the primary `http_bind`.
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
        let provider = Arc::clone(&self.counters.provider);
        Box::pin(async move {
            if request.method != HttpMethod::Get {
                return Err(HttpInboundError::InvalidInput(
                    "metrics endpoint only accepts GET".into(),
                ));
            }
            let body = render_prometheus(&*provider);
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

/// Format `MetricSnapshot` values as Prometheus text exposition.
fn render_prometheus(provider: &dyn MetricsProvider) -> String {
    let mut out = String::new();
    for snap in provider.export() {
        let type_str = match snap.metric_type {
            MetricType::Counter   => "counter",
            MetricType::Gauge     => "gauge",
            MetricType::Histogram => "gauge", // exported as gauge via export()
        };
        out.push_str(&format!("# TYPE {} {}\n", snap.name, type_str));
        let labels = if snap.labels.is_empty() {
            String::new()
        } else {
            let parts: Vec<String> = snap.labels.iter()
                .map(|(k, v)| format!("{k}=\"{v}\""))
                .collect();
            format!("{{{}}}", parts.join(","))
        };
        out.push_str(&format!("{}{} {}\n", snap.name, labels, snap.value));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use swe_observ_metrics::create_local_metrics_backend;
    use crate::api::load_monitor::LoadCounters;

    fn handler_with_data() -> MetricsHandler {
        let provider = Arc::new(create_local_metrics_backend());
        provider.record_counter("edge_requests_total", 42.0, &[]);
        provider.record_gauge("edge_requests_active", 7.0, &[]);
        let counters = Arc::new(LoadCounters::new(provider));
        MetricsHandler::new(counters)
    }

    /// @covers: MetricsHandler::handle — GET returns 200 with Prometheus body
    #[tokio::test]
    async fn test_handle_get_returns_prometheus_text_with_recorded_metrics() {
        let h    = handler_with_data();
        let req  = HttpRequest::get("/metrics");
        let resp = h.handle(req, RequestContext::unauthenticated()).await.unwrap();
        assert_eq!(resp.status, 200);
        let ct   = resp.header("content-type").map(str::to_owned);
        let body = String::from_utf8(resp.body).unwrap();
        assert!(body.contains("edge_requests_total"), "missing counter in: {body}");
        assert!(body.contains("edge_requests_active"), "missing gauge in: {body}");
        assert_eq!(ct.as_deref(), Some("text/plain; version=0.0.4; charset=utf-8"));
    }

    /// @covers: MetricsHandler::handle — POST returns InvalidInput
    #[tokio::test]
    async fn test_handle_non_get_returns_invalid_input_error() {
        let h   = handler_with_data();
        let err = h.handle(HttpRequest::post("/metrics"), RequestContext::unauthenticated())
            .await.unwrap_err();
        assert!(matches!(err, HttpInboundError::InvalidInput(_)));
    }

    /// @covers: render_prometheus
    #[test]
    fn test_render_prometheus_formats_type_line_and_value() {
        let provider = create_local_metrics_backend();
        provider.record_counter("my_counter", 5.0, &[]);
        let out = render_prometheus(&provider);
        assert!(out.contains("# TYPE my_counter counter"));
        assert!(out.contains("my_counter 5"));
    }

    /// @covers: MetricsHandler::health_check
    #[tokio::test]
    async fn test_health_check_returns_healthy() {
        let h  = handler_with_data();
        let hc = h.health_check().await.unwrap();
        assert!(hc.healthy);
    }
}
