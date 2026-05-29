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
    #[serde(default = "ProfileSpec::default_kill_on_job_close")]
    pub kill_on_job_close: bool,
}

impl ProfileSpec {
    /// Default value for [`kill_on_job_close`] — always `true`.
    ///
    /// Used as a serde default function: `#[serde(default = "ProfileSpec::default_kill_on_job_close")]`.
    ///
    /// [`kill_on_job_close`]: ProfileSpec::kill_on_job_close
    pub fn default_kill_on_job_close() -> bool {
        true
    }
}
