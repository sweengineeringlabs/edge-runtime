//! `ProfileSpec` — TOML-deserialized descriptor for a single isolation profile.

/// Descriptor for one named isolation profile in `subprocess_policy.toml`.
///
/// `kind` selects the implementation; remaining fields are kind-specific.
/// Unknown fields are rejected (`deny_unknown_fields`) so TOML typos fail
/// loudly rather than silently reverting to defaults.
///
/// | Kind | Applicable fields |
/// |---|---|
/// | `"noop"` | (none) |
/// | `"seccomp"` | `allowed_syscalls` |
/// | `"job_object"` | `cpu_rate_hundredths`, `memory_limit_bytes`, `kill_on_job_close` |
///
/// # Examples
///
/// ```rust
/// use swe_edge_runtime_isolator::ProfileSpec;
///
/// // No-op profile — no OS restrictions.
/// let spec = ProfileSpec {
///     kind: "noop".to_string(),
///     allowed_syscalls: vec![],
///     cpu_rate_hundredths: 0,
///     memory_limit_bytes: 0,
///     kill_on_job_close: ProfileSpec::default_kill_on_job_close(),
/// };
/// assert_eq!(spec.kind, "noop");
///
/// // Seccomp profile — allowlist syscalls.
/// let spec = ProfileSpec {
///     kind: "seccomp".to_string(),
///     allowed_syscalls: vec!["read".to_string(), "write".to_string(), "exit_group".to_string()],
///     cpu_rate_hundredths: 0,
///     memory_limit_bytes: 0,
///     kill_on_job_close: true,
/// };
/// assert_eq!(spec.allowed_syscalls.len(), 3);
/// ```
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
    #[serde(default = "ProfileSpec::default_kill_on_job_close")]
    pub kill_on_job_close: bool,
}

impl ProfileSpec {
    /// Default value for [`kill_on_job_close`] — always `true`.
    ///
    /// Used as a serde default function: `#[serde(default = "ProfileSpec::default_kill_on_job_close")]`.
    /// Returns `true` so that child processes are always cleaned up when the
    /// Job Object handle drops, preventing orphaned subprocesses.
    ///
    /// [`kill_on_job_close`]: ProfileSpec::kill_on_job_close
    ///
    /// # Examples
    ///
    /// ```rust
    /// use swe_edge_runtime_isolator::ProfileSpec;
    /// assert!(ProfileSpec::default_kill_on_job_close());
    /// ```
    pub fn default_kill_on_job_close() -> bool {
        true
    }
}
