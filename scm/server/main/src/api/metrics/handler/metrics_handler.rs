//! MetricsHandler interface — re-exported from traits.

pub use crate::api::metrics::traits::metrics_handler::MetricsHandler;

/// Default HTTP path at which Prometheus metrics are served.
pub const DEFAULT_METRICS_PATH: &str = "/metrics";
