//! `SeccompIsolator` — `seccomp-bpf` syscall filter (Linux 3.5+).
//!
//! Requires the `seccomp` feature flag and the `seccompiler` crate.
//! The filter is compiled once at construction and installed in the child
//! process after `fork()` via `Command::pre_exec`, before `exec()`.

#![cfg(all(target_os = "linux", feature = "seccomp"))]

pub(crate) mod seccomp_isolator;
pub(crate) use seccomp_isolator::SeccompIsolator;
