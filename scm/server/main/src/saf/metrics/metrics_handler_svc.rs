//! SAF — `MetricsHandler` public service surface.
pub use crate::api::metrics::traits::metrics_handler::MetricsHandler;
/// Identifies the `MetricsHandler` SAF contract in this crate.
pub const METRICS_HANDLER_SVC: &str = "metrics_handler";
