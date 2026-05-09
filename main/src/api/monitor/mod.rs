//! Load monitor — type declarations (one type per file).

pub(crate) mod autoscale_policy;
pub(crate) mod counters;
pub(crate) mod metrics_config;
pub(crate) mod ring_buffer;
pub(crate) mod shared_counters;

pub use autoscale_policy::AutoscalePolicy;
pub use counters::LoadCounters;
pub use metrics_config::MetricsConfig;
pub use ring_buffer::RingBuffer;
pub use shared_counters::SharedCounters;
