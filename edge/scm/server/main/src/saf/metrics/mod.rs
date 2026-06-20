//! SAF — metrics service surface.
mod metrics_exporter_svc;
mod metrics_handler_svc;
pub use metrics_exporter_svc::{MetricsExporter, METRICS_EXPORTER_SVC};
pub use metrics_handler_svc::{MetricsHandler, METRICS_HANDLER_SVC};
