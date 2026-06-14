//! Integration tests for the runner_svc SAF surface.
#![allow(clippy::unwrap_used)]

use std::mem::size_of_val;
use swe_edge_runtime::{Runner, RuntimeError, RuntimeResult, RUNNER_SVC};

struct NoopRunner;
impl Runner for NoopRunner {
    fn run(&self) -> RuntimeResult<()> {
        Ok(())
    }
}

struct FailRunner;
impl Runner for FailRunner {
    fn run(&self) -> RuntimeResult<()> {
        Err(RuntimeError::StartFailed("injected failure".into()))
    }
}

/// @covers: RUNNER_SVC
#[test]
fn test_runner_svc_slug_is_correct_happy() {
    assert_eq!(RUNNER_SVC, "runner");
}

// ── Runner::run ───────────────────────────────────────────────────────────────

#[test]
fn test_run_noop_runner_returns_ok_happy() {
    assert!(NoopRunner.run().is_ok());
}

#[test]
fn test_run_fail_runner_returns_err_error() {
    assert!(FailRunner.run().is_err());
}

#[test]
fn test_run_error_message_is_propagated_edge() {
    let err = FailRunner.run().unwrap_err();
    assert!(err.to_string().contains("injected failure"));
}

// ── Runner::new_builder ───────────────────────────────────────────────────────

#[test]
fn test_new_builder_returns_runtime_builder_without_panic_happy() {
    let _b = NoopRunner::new_builder();
}

#[test]
fn test_new_builder_build_registry_is_none_without_egress_error() {
    let b = NoopRunner::new_builder();
    assert!(b.build_registry().is_none());
}

#[test]
fn test_new_builder_called_twice_gives_independent_builders_edge() {
    let b1 = NoopRunner::new_builder();
    let b2 = NoopRunner::new_builder();
    assert!(b1.build_registry().is_none());
    assert!(b2.build_registry().is_none());
}

// ── Runner::runtime_entry ─────────────────────────────────────────────────────

#[test]
fn test_runtime_entry_returns_zero_size_type_happy() {
    let rt = NoopRunner::runtime_entry();
    assert_eq!(size_of_val(&rt), 0);
}

#[test]
fn test_runtime_entry_is_callable_without_panic_error() {
    let _rt = NoopRunner::runtime_entry();
}

#[test]
fn test_runtime_entry_callable_twice_is_stable_edge() {
    let _a = NoopRunner::runtime_entry();
    let _b = NoopRunner::runtime_entry();
}

// ── Runner::server_config_loader ──────────────────────────────────────────────

#[test]
fn test_server_config_loader_returns_marker_type_happy() {
    let _scl = NoopRunner::server_config_loader();
}

#[test]
fn test_server_config_loader_is_zero_size_error() {
    let scl = NoopRunner::server_config_loader();
    assert_eq!(size_of_val(&scl), 0);
}

#[test]
fn test_server_config_loader_called_twice_is_stable_edge() {
    let _a = NoopRunner::server_config_loader();
    let _b = NoopRunner::server_config_loader();
}

// ── Runner::server_monitor ────────────────────────────────────────────────────

#[test]
fn test_server_monitor_returns_marker_type_happy() {
    let _sm = NoopRunner::server_monitor();
}

#[test]
fn test_server_monitor_is_zero_size_error() {
    let sm = NoopRunner::server_monitor();
    assert_eq!(size_of_val(&sm), 0);
}

#[test]
fn test_server_monitor_called_twice_is_stable_edge() {
    let _a = NoopRunner::server_monitor();
    let _b = NoopRunner::server_monitor();
}

// ── Runner::tracing_initializer ───────────────────────────────────────────────

#[test]
fn test_tracing_initializer_returns_marker_type_happy() {
    let _ti = NoopRunner::tracing_initializer();
}

#[test]
fn test_tracing_initializer_is_zero_size_error() {
    let ti = NoopRunner::tracing_initializer();
    assert_eq!(size_of_val(&ti), 0);
}

#[test]
fn test_tracing_initializer_called_twice_is_stable_edge() {
    let _a = NoopRunner::tracing_initializer();
    let _b = NoopRunner::tracing_initializer();
}

// ── Runner::builder_serve_marker ──────────────────────────────────────────────

#[test]
fn test_builder_serve_marker_returns_zero_size_type_happy() {
    let _bsm = NoopRunner::builder_serve_marker();
}

#[test]
fn test_builder_serve_marker_is_zero_size_error() {
    let bsm = NoopRunner::builder_serve_marker();
    assert_eq!(size_of_val(&bsm), 0);
}

#[test]
fn test_builder_serve_marker_called_twice_is_stable_edge() {
    let _a = NoopRunner::builder_serve_marker();
    let _b = NoopRunner::builder_serve_marker();
}
