//! Tracing subscriber initialisation — delegates to swe-edge-observ-config.

#[cfg(feature = "observability")]
use swe_edge_observ_config::TracingConfig;

/// Install a `tracing-subscriber` driven by `config`.
///
/// Requires the `observability` feature. Idempotent — safe to call multiple
/// times. Does nothing when `config.enabled` is `false`. `RUST_LOG` overrides
/// `config.level` and `config.filter`.
#[cfg(feature = "observability")]
pub fn init_tracing(config: &TracingConfig) {
    swe_edge_observ_config::init_tracing(config);
}
