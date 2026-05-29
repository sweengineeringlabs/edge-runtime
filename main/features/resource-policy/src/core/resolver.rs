//! `ResourceLimitsResolver` — pure multi-level resource limit resolver.

use crate::api::limits::ResourceLimits;
use crate::api::policy::ResourcePolicy;

/// Resolves [`ResourceLimits`] from an ordered priority chain.
///
/// Layers are added in priority order — the first `Some` value for each
/// field wins. Fields absent from all layers fall back to the `defaults`
/// supplied to [`resolve_with_defaults`].
///
/// This is a pure value object: no I/O, no config loading, fully testable
/// without spawning any process.
///
/// # Example
///
/// ```rust
/// use swe_edge_runtime_resource_policy::{ResourceLimits, ResourceLimitsResolver, ResourcePolicy};
///
/// # fn main() {
/// let step   = ResourceLimits { timeout_ms: Some(2_000), ..Default::default() };
/// let agent  = ResourceLimits { timeout_ms: Some(10_000), cpu_time_ms: Some(5_000), ..Default::default() };
/// let floor  = ResourcePolicy {
///     name: "default".into(), timeout_ms: 30_000, output_bytes_cap: 1_048_576,
///     cpu_time_ms: 0, memory_bytes: 0,
/// };
///
/// let resolved = ResourceLimitsResolver::new()
///     .layer(step)    // highest priority
///     .layer(agent)
///     .resolve_with_defaults(&floor);
///
/// assert_eq!(resolved.timeout_ms, 2_000);         // step wins
/// assert_eq!(resolved.cpu_time_ms, 5_000);        // agent fills (step had None)
/// assert_eq!(resolved.output_bytes_cap, 1_048_576); // floor fills
/// # }
/// ```
///
/// [`resolve_with_defaults`]: ResourceLimitsResolver::resolve_with_defaults
pub struct ResourceLimitsResolver {
    layers: Vec<ResourceLimits>,
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
                break; // all fields resolved — skip remaining layers
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

#[cfg(test)]
mod tests {
    use super::*;

    fn floor() -> ResourcePolicy {
        ResourcePolicy {
            name: "floor".into(),
            timeout_ms: 30_000,
            output_bytes_cap: 1_048_576,
            cpu_time_ms: 0,
            memory_bytes: 0,
        }
    }

    #[test]
    fn test_resolver_no_layers_returns_defaults() {
        let resolved = ResourceLimitsResolver::new().resolve_with_defaults(&floor());
        assert_eq!(resolved.timeout_ms, 30_000);
        assert_eq!(resolved.output_bytes_cap, 1_048_576);
        assert_eq!(resolved.cpu_time_ms, 0);
        assert_eq!(resolved.memory_bytes, 0);
    }

    #[test]
    fn test_resolver_first_layer_wins_over_defaults() {
        let layer = ResourceLimits {
            timeout_ms: Some(5_000),
            ..Default::default()
        };
        let resolved = ResourceLimitsResolver::new()
            .layer(layer)
            .resolve_with_defaults(&floor());
        assert_eq!(resolved.timeout_ms, 5_000);
        assert_eq!(resolved.output_bytes_cap, 1_048_576); // floor fills
    }

    #[test]
    fn test_resolver_higher_priority_layer_wins_over_lower() {
        let high = ResourceLimits {
            timeout_ms: Some(1_000),
            ..Default::default()
        };
        let low = ResourceLimits {
            timeout_ms: Some(9_000),
            ..Default::default()
        };
        let resolved = ResourceLimitsResolver::new()
            .layer(high)
            .layer(low)
            .resolve_with_defaults(&floor());
        assert_eq!(resolved.timeout_ms, 1_000);
    }

    #[test]
    fn test_resolver_later_layer_fills_missing_fields_from_earlier() {
        let step = ResourceLimits {
            timeout_ms: Some(2_000),
            ..Default::default()
        };
        let agent = ResourceLimits {
            cpu_time_ms: Some(5_000),
            ..Default::default()
        };
        let resolved = ResourceLimitsResolver::new()
            .layer(step)
            .layer(agent)
            .resolve_with_defaults(&floor());
        assert_eq!(resolved.timeout_ms, 2_000); // step
        assert_eq!(resolved.cpu_time_ms, 5_000); // agent fills
        assert_eq!(resolved.output_bytes_cap, 1_048_576); // floor
    }

    #[test]
    fn test_resolver_zero_cpu_time_is_preserved_not_skipped() {
        let layer = ResourceLimits {
            cpu_time_ms: Some(0),
            ..Default::default()
        };
        let mut f = floor();
        f.cpu_time_ms = 10_000; // floor has a non-zero value
        let resolved = ResourceLimitsResolver::new()
            .layer(layer)
            .resolve_with_defaults(&f);
        assert_eq!(resolved.cpu_time_ms, 0); // Some(0) must win over floor's 10_000
    }
}
