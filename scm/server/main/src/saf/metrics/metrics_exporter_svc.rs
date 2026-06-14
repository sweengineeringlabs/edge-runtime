//! SAF — `MetricsExporter` public service surface.
pub use crate::api::metrics::traits::metrics_exporter::MetricsExporter;
/// Identifies the `MetricsExporter` SAF contract in this crate.
pub const METRICS_EXPORTER_SVC: &str = "metrics_exporter";
