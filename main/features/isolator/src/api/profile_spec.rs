//! `ProfileSpec` — TOML-deserialized descriptor for a single isolation profile.

/// Descriptor for one named isolation profile in `subprocess_policy.toml`.
///
/// `kind` selects the implementation; remaining fields are kind-specific.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProfileSpec {
    /// Implementation kind: `"noop"`, `"seccomp"`, or `"job_object"`.
    pub kind: String,

    /// Allowed syscall names — only meaningful when `kind = "seccomp"`.
    #[serde(default)]
    pub allowed_syscalls: Vec<String>,

    /// CPU rate limit as hundredths of one CPU — only meaningful when
    /// `kind = "job_object"`. `0` means unlimited.
    #[serde(default)]
    pub cpu_rate_hundredths: u32,

    /// Maximum working-set memory in bytes — only meaningful when
    /// `kind = "job_object"`. `0` means unlimited.
    #[serde(default)]
    pub memory_limit_bytes: u64,

    /// Kill all processes in the Job Object when the runner drops —
    /// only meaningful when `kind = "job_object"`.
    #[serde(default = "default_kill_on_job_close")]
    pub kill_on_job_close: bool,
}

fn default_kill_on_job_close() -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_spec_deserializes_noop() {
        let toml = r#"kind = "noop""#;
        let spec: ProfileSpec = toml::from_str(toml).unwrap();
        assert_eq!(spec.kind, "noop");
        assert!(spec.allowed_syscalls.is_empty());
    }

    #[test]
    fn test_profile_spec_deserializes_seccomp_with_syscalls() {
        let toml = r#"
            kind = "seccomp"
            allowed_syscalls = ["read", "write", "exit"]
        "#;
        let spec: ProfileSpec = toml::from_str(toml).unwrap();
        assert_eq!(spec.kind, "seccomp");
        assert_eq!(spec.allowed_syscalls.len(), 3);
    }

    #[test]
    fn test_profile_spec_kill_on_job_close_defaults_true() {
        let toml = r#"kind = "job_object""#;
        let spec: ProfileSpec = toml::from_str(toml).unwrap();
        assert!(spec.kill_on_job_close);
    }
}
