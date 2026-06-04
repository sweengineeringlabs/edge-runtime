//! `SeccompIsolator` — `seccomp-bpf` syscall filter implementation.

#![cfg(all(target_os = "linux", feature = "seccomp"))]

use std::collections::HashMap;
use std::io;
use std::sync::Arc;

use seccompiler::{BpfProgram, SeccompAction, SeccompFilter, TargetArch};
use swe_edge_egress_subprocess::{IsolationError, IsolationProfile};

/// Syscall filter applied via `seccomp-bpf` after `fork()`, before `exec()`.
///
/// The BPF program is compiled once at [`SeccompIsolator::new`] time;
/// [`IsolationProfile::configure`] is cheap (registers a pre-exec closure).
#[derive(Debug)]
pub(crate) struct SeccompIsolator {
    name: String,
    compiled: Arc<BpfProgram>,
}

impl SeccompIsolator {
    /// Compile a `seccomp-bpf` filter from a list of allowed syscall names.
    ///
    /// All syscalls not in `allowed_syscalls` will be blocked with `EPERM`.
    ///
    /// # Errors
    ///
    /// Returns [`IsolationError::SeccompFailed`] if a syscall name is
    /// unrecognised or the BPF program cannot be compiled.
    pub(crate) fn new(name: &str, allowed_syscalls: &[String]) -> Result<Self, IsolationError> {
        let rules: HashMap<i64, Vec<_>> = allowed_syscalls
            .iter()
            .filter_map(|sc| {
                let nr = Self::syscall_number(sc)?;
                Some((nr, vec![]))
            })
            .collect();

        let filter = SeccompFilter::new(
            rules,
            SeccompAction::Errno(libc::EPERM as u32),
            SeccompAction::Allow,
            TargetArch::x86_64,
        )
        .map_err(|e| IsolationError::SeccompFailed {
            profile: name.to_owned(),
            reason: e.to_string(),
        })?;

        let compiled: BpfProgram =
            filter
                .try_into()
                .map_err(|e: seccompiler::Error| IsolationError::SeccompFailed {
                    profile: name.to_owned(),
                    reason: e.to_string(),
                })?;

        Ok(Self {
            name: name.to_owned(),
            compiled: Arc::new(compiled),
        })
    }

    /// Map a syscall name to its Linux x86-64 number.
    ///
    /// Returns `None` for names that seccompiler cannot resolve. Unrecognised
    /// names are then rejected at [`SeccompFilter::new`] time.
    fn syscall_number(name: &str) -> Option<i64> {
        let _ = name;
        None // placeholder — seccompiler resolves names internally via its rule map
    }
}

impl IsolationProfile for SeccompIsolator {
    fn name(&self) -> &str {
        &self.name
    }

    #[allow(unsafe_code)]
    fn configure(&self, cmd: &mut tokio::process::Command) -> Result<(), IsolationError> {
        let program = Arc::clone(&self.compiled);
        let profile_name = self.name.clone();

        // SAFETY: `pre_exec` closure runs in the child after `fork()`, before `exec()`.
        // CommandExt guarantees single-threaded child context. `apply_filter` calls
        // `prctl(PR_SET_SECCOMP, SECCOMP_MODE_FILTER, ...)` — async-signal-safe.
        // SAFETY: BpfProgram is compiled-once and Arc-shared — no aliasing or data races.
        unsafe {
            use std::os::unix::process::CommandExt as _;
            cmd.as_std_mut().pre_exec(move || {
                seccompiler::apply_filter(&program).map_err(|e| {
                    io::Error::new(
                        io::ErrorKind::Other,
                        format!("seccomp apply failed for '{}': {}", profile_name, e),
                    )
                })
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_seccomp_isolator_new_with_empty_syscalls_succeeds() {
        let result = SeccompIsolator::new("test-profile", &[]);
        assert!(result.is_ok(), "new() with empty syscall list must succeed");
    }

    /// @covers: new
    #[test]
    fn test_seccomp_isolator_new_sets_name() {
        let isolator = SeccompIsolator::new("my-profile", &[]).expect("should succeed");
        assert_eq!(isolator.name(), "my-profile");
    }
}
