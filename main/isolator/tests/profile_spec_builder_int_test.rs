//! Integration tests for `ProfileSpecBuilder`.

use swe_edge_runtime_isolator::ProfileSpecBuilder;

/// @covers: ProfileSpecBuilder::new
#[test]
fn test_profile_spec_builder_new_defaults_to_noop_kind() {
    let spec = ProfileSpecBuilder::new().build();
    assert_eq!(spec.kind, "noop");
}

/// @covers: ProfileSpecBuilder::kind
#[test]
fn test_profile_spec_builder_kind_sets_kind() {
    let spec = ProfileSpecBuilder::new().kind("seccomp").build();
    assert_eq!(spec.kind, "seccomp");
}

/// @covers: ProfileSpecBuilder::allowed_syscalls
#[test]
fn test_profile_spec_builder_allowed_syscalls_sets_syscalls() {
    let syscalls = vec!["read".to_owned(), "write".to_owned()];
    let spec = ProfileSpecBuilder::new()
        .kind("seccomp")
        .allowed_syscalls(syscalls.clone())
        .build();
    assert_eq!(spec.allowed_syscalls, syscalls);
}

/// @covers: ProfileSpecBuilder::kill_on_job_close
#[test]
fn test_profile_spec_builder_kill_on_job_close_defaults_to_true() {
    let spec = ProfileSpecBuilder::new().build();
    assert!(spec.kill_on_job_close);
}
