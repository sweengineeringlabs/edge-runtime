//! SAF — `HttpLoadMonitor` public service surface.
pub use crate::api::monitor::http_load_monitor::HttpLoadMonitor;
/// Identifies the `HttpLoadMonitor` SAF contract in this crate.
pub const HTTP_LOAD_MONITOR_SVC: &str = "http_load_monitor";
