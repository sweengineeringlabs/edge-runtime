//! `SharedCounters` — `Arc`-wrapped `LoadCounters` shared across request handlers.

use std::sync::Arc;
use crate::api::monitor::counters::LoadCounters;

/// Shared handle passed between the monitor wrappers and the metrics server.
pub type SharedCounters = Arc<LoadCounters>;
