//! `ResourcePolicy` — fully-resolved, named resource policy.

/// A fully-resolved, named resource policy for subprocess execution.
///
/// All fields are concrete — `ResourcePolicy` is the result of resolution,
/// not an input to it. Constructed by [`ResourceLimitsResolver::resolve_with_defaults`]
/// or loaded directly from TOML config via [`ResourcePolicyConfig::get`].
///
/// No `Default` impl — policies are data in config files, not source literals.
///
/// [`ResourceLimitsResolver::resolve_with_defaults`]: crate::ResourceLimitsResolver::resolve_with_defaults
/// [`ResourcePolicyConfig::get`]: crate::ResourcePolicyConfig::get
///
/// # Examples
///
/// ```rust
/// use swe_edge_runtime_resource_policy::ResourcePolicy;
///
/// // Construct a policy for testing (in production, load from config).
/// let policy = ResourcePolicy {
///     name: "default".to_string(),
///     timeout_ms: 30_000,
///     output_bytes_cap: 1_048_576,
///     cpu_time_ms: 0,      // unlimited
///     memory_bytes: 0,     // unlimited
/// };
///
/// assert_eq!(policy.name, "default");
/// assert_eq!(policy.timeout_ms, 30_000);
/// assert_eq!(policy.cpu_time_ms, 0); // 0 = unlimited
/// ```
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ResourcePolicy {
    /// Stable identifier used in audit logs and error messages.
    pub name: String,

    /// Wall-clock timeout in milliseconds. Process killed on expiry.
    pub timeout_ms: u64,

    /// Maximum combined stdout + stderr bytes buffered.
    pub output_bytes_cap: u64,

    /// CPU time limit in milliseconds (user + system). `0` = unlimited.
    pub cpu_time_ms: u64,

    /// Maximum virtual address space in bytes. `0` = unlimited.
    pub memory_bytes: u64,
}

impl ResourcePolicy {
    /// Apply this policy's limits to a [`SubprocessArgs`] builder,
    /// filling only fields that the caller left as `None`.
    ///
    /// This is the injection step used by [`ResourcePolicyRunner`].
    ///
    /// [`SubprocessArgs`]: swe_edge_egress_subprocess::SubprocessArgs
    /// [`ResourcePolicyRunner`]: crate::ResourcePolicyRunner
    pub fn inject_into(&self, args: &mut swe_edge_egress_subprocess::SubprocessArgs) {
        args.timeout_ms.get_or_insert(self.timeout_ms);
        args.output_bytes_cap.get_or_insert(self.output_bytes_cap);
        args.cpu_time_ms.get_or_insert(self.cpu_time_ms);
        args.memory_bytes.get_or_insert(self.memory_bytes);
    }
}
