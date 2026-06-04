//! `ResourceLimitsBuilder` — fluent builder for [`ResourceLimits`].

use super::resource_limits::ResourceLimits;

/// Fluent builder for [`ResourceLimits`].
///
/// All setters are optional — unset fields remain `None` (defer to next layer).
///
/// # Example
///
/// ```rust
/// use swe_edge_runtime_resource_policy::ResourceLimitsBuilder;
///
/// let limits = ResourceLimitsBuilder::new()
///     .timeout_ms(5_000)
///     .output_bytes_cap(1_048_576)
///     .build();
/// assert_eq!(limits.timeout_ms, Some(5_000));
/// ```
#[derive(Debug, Default)]
pub struct ResourceLimitsBuilder {
    inner: ResourceLimits,
}

impl ResourceLimitsBuilder {
    /// Create a new builder with all fields `None`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the wall-clock timeout in milliseconds.
    pub fn timeout_ms(mut self, v: u64) -> Self {
        self.inner.timeout_ms = Some(v);
        self
    }

    /// Set the maximum combined stdout + stderr bytes.
    pub fn output_bytes_cap(mut self, v: u64) -> Self {
        self.inner.output_bytes_cap = Some(v);
        self
    }

    /// Set the CPU time limit in milliseconds.
    pub fn cpu_time_ms(mut self, v: u64) -> Self {
        self.inner.cpu_time_ms = Some(v);
        self
    }

    /// Set the maximum virtual address space in bytes.
    pub fn memory_bytes(mut self, v: u64) -> Self {
        self.inner.memory_bytes = Some(v);
        self
    }

    /// Consume the builder and return the configured [`ResourceLimits`].
    pub fn build(self) -> ResourceLimits {
        self.inner
    }
}
