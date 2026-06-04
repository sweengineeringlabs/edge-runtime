//! `ResourceLimitsResolver` — pure multi-level resource limit resolver.

use super::resource_limits::ResourceLimits;
use super::resource_policy::ResourcePolicy;

/// Resolves [`ResourceLimits`] from an ordered priority chain.
///
/// Layers are added in priority order — the first `Some` value for each
/// field wins. Fields absent from all layers fall back to the `defaults`
/// supplied to [`resolve_with_defaults`].
///
/// This is a pure value object: no I/O, no config loading, fully testable
/// without spawning any process.
///
/// [`resolve_with_defaults`]: ResourceLimitsResolver::resolve_with_defaults
pub struct ResourceLimitsResolver {
    pub(crate) layers: Vec<ResourceLimits>,
}

impl ResourceLimitsResolver {
    /// Create an empty resolver.
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    /// Add a layer. Layers added earlier have higher priority.
    pub fn layer(mut self, limits: ResourceLimits) -> Self {
        self.layers.push(limits);
        self
    }

    /// Resolve all fields, filling any remaining `None` from `defaults`.
    ///
    /// Returns a [`ResourcePolicy`] with no `None` fields — all four
    /// dimensions are concrete values.
    pub fn resolve_with_defaults(self, defaults: &ResourcePolicy) -> ResourcePolicy {
        let mut timeout_ms = None;
        let mut output_bytes_cap = None;
        let mut cpu_time_ms = None;
        let mut memory_bytes = None;

        for layer in &self.layers {
            if timeout_ms.is_none() {
                timeout_ms = layer.timeout_ms;
            }
            if output_bytes_cap.is_none() {
                output_bytes_cap = layer.output_bytes_cap;
            }
            if cpu_time_ms.is_none() {
                cpu_time_ms = layer.cpu_time_ms;
            }
            if memory_bytes.is_none() {
                memory_bytes = layer.memory_bytes;
            }
            if timeout_ms.is_some()
                && output_bytes_cap.is_some()
                && cpu_time_ms.is_some()
                && memory_bytes.is_some()
            {
                break;
            }
        }

        ResourcePolicy {
            name: defaults.name.clone(),
            timeout_ms: timeout_ms.unwrap_or(defaults.timeout_ms),
            output_bytes_cap: output_bytes_cap.unwrap_or(defaults.output_bytes_cap),
            cpu_time_ms: cpu_time_ms.unwrap_or(defaults.cpu_time_ms),
            memory_bytes: memory_bytes.unwrap_or(defaults.memory_bytes),
        }
    }
}

impl Default for ResourceLimitsResolver {
    fn default() -> Self {
        Self::new()
    }
}
