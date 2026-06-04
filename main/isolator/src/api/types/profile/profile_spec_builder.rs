//! `ProfileSpecBuilder` — fluent builder for [`ProfileSpec`].

use super::profile_spec::ProfileSpec;

/// Fluent builder for [`ProfileSpec`].
///
/// All fields have sensible defaults:
/// - `kind` defaults to `"noop"`
/// - `allowed_syscalls` defaults to empty
/// - `cpu_rate_hundredths` defaults to `0` (unlimited)
/// - `memory_limit_bytes` defaults to `0` (unlimited)
/// - `kill_on_job_close` defaults to `true`
pub struct ProfileSpecBuilder {
    kind: String,
    allowed_syscalls: Vec<String>,
    cpu_rate_hundredths: u32,
    memory_limit_bytes: u64,
    kill_on_job_close: bool,
}

impl ProfileSpecBuilder {
    /// Create a new builder with default values (`kind = "noop"`).
    pub fn new() -> Self {
        Self {
            kind: "noop".to_owned(),
            allowed_syscalls: Vec::new(),
            cpu_rate_hundredths: 0,
            memory_limit_bytes: 0,
            kill_on_job_close: true,
        }
    }

    /// Set the profile kind (e.g. `"noop"`, `"seccomp"`, `"job_object"`).
    pub fn kind(mut self, kind: impl Into<String>) -> Self {
        self.kind = kind.into();
        self
    }

    /// Set the allowed syscall names (only meaningful for `kind = "seccomp"`).
    pub fn allowed_syscalls(mut self, syscalls: Vec<String>) -> Self {
        self.allowed_syscalls = syscalls;
        self
    }

    /// Set the CPU rate limit in hundredths of one CPU (only meaningful for `kind = "job_object"`).
    pub fn cpu_rate_hundredths(mut self, rate: u32) -> Self {
        self.cpu_rate_hundredths = rate;
        self
    }

    /// Set the maximum working-set memory in bytes (only meaningful for `kind = "job_object"`).
    pub fn memory_limit_bytes(mut self, limit: u64) -> Self {
        self.memory_limit_bytes = limit;
        self
    }

    /// Set whether to kill all processes in the Job Object when the runner drops.
    pub fn kill_on_job_close(mut self, kill: bool) -> Self {
        self.kill_on_job_close = kill;
        self
    }

    /// Consume the builder and return the [`ProfileSpec`].
    pub fn build(self) -> ProfileSpec {
        ProfileSpec {
            kind: self.kind,
            allowed_syscalls: self.allowed_syscalls,
            cpu_rate_hundredths: self.cpu_rate_hundredths,
            memory_limit_bytes: self.memory_limit_bytes,
            kill_on_job_close: self.kill_on_job_close,
        }
    }
}

impl Default for ProfileSpecBuilder {
    fn default() -> Self {
        Self::new()
    }
}
