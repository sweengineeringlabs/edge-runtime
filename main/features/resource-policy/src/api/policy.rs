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
    /// Apply this policy's limits to a [`ProcessArgs`] builder,
    /// filling only fields that the caller left as `None`.
    ///
    /// This is the injection step used by [`ResourcePolicyRunner`].
    ///
    /// [`ProcessArgs`]: swe_edge_egress_subprocess::ProcessArgs
    /// [`ResourcePolicyRunner`]: crate::ResourcePolicyRunner
    pub fn inject_into(&self, args: &mut swe_edge_egress_subprocess::ProcessArgs) {
        args.timeout_ms.get_or_insert(self.timeout_ms);
        args.output_bytes_cap.get_or_insert(self.output_bytes_cap);
        args.cpu_time_ms.get_or_insert(self.cpu_time_ms);
        args.memory_bytes.get_or_insert(self.memory_bytes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use swe_edge_egress_subprocess::ProcessArgs;

    fn stub_policy() -> ResourcePolicy {
        ResourcePolicy {
            name: "test".into(),
            timeout_ms: 5_000,
            output_bytes_cap: 512,
            cpu_time_ms: 3_000,
            memory_bytes: 1_024,
        }
    }

    /// @covers: inject_into
    #[test]
    fn test_inject_into_fills_none_fields() {
        let policy = stub_policy();
        let mut args = ProcessArgs::builder().argv(vec!["echo".into()]).build();
        policy.inject_into(&mut args);
        assert_eq!(args.timeout_ms, Some(5_000));
        assert_eq!(args.output_bytes_cap, Some(512));
        assert_eq!(args.cpu_time_ms, Some(3_000));
        assert_eq!(args.memory_bytes, Some(1_024));
    }

    /// @covers: inject_into
    #[test]
    fn test_inject_into_does_not_overwrite_existing_values() {
        let policy = stub_policy();
        let mut args = ProcessArgs::builder()
            .argv(vec!["echo".into()])
            .timeout_ms(99_999)
            .build();
        policy.inject_into(&mut args);
        assert_eq!(args.timeout_ms, Some(99_999)); // caller value preserved
        assert_eq!(args.output_bytes_cap, Some(512)); // policy fills the rest
    }
}
