//! `ResourcePolicyBuilder` — fluent builder for [`ResourcePolicy`].

use super::resource_policy::ResourcePolicy;

/// Fluent builder for [`ResourcePolicy`].
///
/// All fields are required — call every setter before [`build`] or the build
/// will use the provided defaults.
///
/// # Example
///
/// ```rust
/// use swe_edge_runtime_resource_policy::ResourcePolicyBuilder;
///
/// let policy = ResourcePolicyBuilder::new()
///     .name("default")
///     .timeout_ms(30_000)
///     .output_bytes_cap(1_048_576)
///     .cpu_time_ms(0)
///     .memory_bytes(0)
///     .build();
/// assert_eq!(policy.name, "default");
/// ```
///
/// [`build`]: ResourcePolicyBuilder::build
#[derive(Debug, Default)]
pub struct ResourcePolicyBuilder {
    name: String,
    timeout_ms: u64,
    output_bytes_cap: u64,
    cpu_time_ms: u64,
    memory_bytes: u64,
}

impl ResourcePolicyBuilder {
    /// Create a new builder with zero-value defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the policy name (used in audit logs and error messages).
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Set the wall-clock timeout in milliseconds.
    pub fn timeout_ms(mut self, v: u64) -> Self {
        self.timeout_ms = v;
        self
    }

    /// Set the maximum combined stdout + stderr bytes.
    pub fn output_bytes_cap(mut self, v: u64) -> Self {
        self.output_bytes_cap = v;
        self
    }

    /// Set the CPU time limit in milliseconds (`0` = unlimited).
    pub fn cpu_time_ms(mut self, v: u64) -> Self {
        self.cpu_time_ms = v;
        self
    }

    /// Set the maximum virtual address space in bytes (`0` = unlimited).
    pub fn memory_bytes(mut self, v: u64) -> Self {
        self.memory_bytes = v;
        self
    }

    /// Consume the builder and return the configured [`ResourcePolicy`].
    pub fn build(self) -> ResourcePolicy {
        ResourcePolicy {
            name: self.name,
            timeout_ms: self.timeout_ms,
            output_bytes_cap: self.output_bytes_cap,
            cpu_time_ms: self.cpu_time_ms,
            memory_bytes: self.memory_bytes,
        }
    }
}
