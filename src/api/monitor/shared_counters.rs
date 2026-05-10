//! `SharedCounters` — `Arc`-wrapped `TrafficCounters` shared across request handlers.

use std::sync::Arc;
use crate::api::monitor::traffic_counters::TrafficCounters;

/// Shared handle passed between the monitor wrappers and the metrics server.
pub type SharedCounters = Arc<TrafficCounters>;
