//! SAF — `GrpcLoadMonitor` public service surface.
pub use crate::api::monitor::grpc_load_monitor::GrpcLoadMonitor;
/// Identifies the `GrpcLoadMonitor` SAF contract in this crate.
pub const GRPC_LOAD_MONITOR_SVC: &str = "grpc_load_monitor";
