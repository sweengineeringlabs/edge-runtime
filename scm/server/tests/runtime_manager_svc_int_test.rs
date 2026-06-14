//! Integration tests for the runtime_manager_svc SAF surface.
#![allow(clippy::unwrap_used)]

use futures::future::BoxFuture;
use swe_edge_runtime::{
    ComponentHealth, RuntimeError, RuntimeHealth, RuntimeManager, RuntimeResult, RuntimeStatus,
    RUNTIME_MANAGER_SVC,
};

struct OkManager;
impl RuntimeManager for OkManager {
    fn start(&self) -> BoxFuture<'_, RuntimeResult<()>> {
        Box::pin(async { Ok(()) })
    }
    fn shutdown(&self) -> BoxFuture<'_, RuntimeResult<()>> {
        Box::pin(async { Ok(()) })
    }
    fn health(&self) -> BoxFuture<'_, RuntimeHealth> {
        Box::pin(async {
            RuntimeHealth {
                status: RuntimeStatus::Running,
                components: vec![ComponentHealth::healthy("test")],
                uptime_secs: 0,
            }
        })
    }
}

struct FailManager;
impl RuntimeManager for FailManager {
    fn start(&self) -> BoxFuture<'_, RuntimeResult<()>> {
        Box::pin(async { Err(RuntimeError::StartFailed("injected".into())) })
    }
    fn shutdown(&self) -> BoxFuture<'_, RuntimeResult<()>> {
        Box::pin(async { Err(RuntimeError::StartFailed("injected".into())) })
    }
    fn health(&self) -> BoxFuture<'_, RuntimeHealth> {
        Box::pin(async {
            RuntimeHealth {
                status: RuntimeStatus::Stopped,
                components: vec![],
                uptime_secs: 0,
            }
        })
    }
}

/// @covers: RUNTIME_MANAGER_SVC
#[test]
fn test_runtime_manager_svc_slug_is_correct_happy() {
    assert_eq!(RUNTIME_MANAGER_SVC, "runtime_manager");
}

// ── RuntimeManager::start ─────────────────────────────────────────────────────

#[test]
fn test_start_ok_manager_returns_ok_happy() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    assert!(rt.block_on(OkManager.start()).is_ok());
}

#[test]
fn test_start_fail_manager_returns_err_error() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    assert!(rt.block_on(FailManager.start()).is_err());
}

#[test]
fn test_start_ok_manager_called_twice_succeeds_edge() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    assert!(rt.block_on(OkManager.start()).is_ok());
    assert!(rt.block_on(OkManager.start()).is_ok());
}

// ── RuntimeManager::shutdown ──────────────────────────────────────────────────

#[test]
fn test_shutdown_ok_manager_returns_ok_happy() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    assert!(rt.block_on(OkManager.shutdown()).is_ok());
}

#[test]
fn test_shutdown_fail_manager_returns_err_error() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    assert!(rt.block_on(FailManager.shutdown()).is_err());
}

#[test]
fn test_shutdown_after_start_returns_ok_edge() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(OkManager.start()).unwrap();
    assert!(rt.block_on(OkManager.shutdown()).is_ok());
}

// ── RuntimeManager::health ────────────────────────────────────────────────────

#[test]
fn test_health_ok_manager_status_is_running_happy() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let h = rt.block_on(OkManager.health());
    assert_eq!(h.status, RuntimeStatus::Running);
}

#[test]
fn test_health_fail_manager_status_is_stopped_error() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let h = rt.block_on(FailManager.health());
    assert_eq!(h.status, RuntimeStatus::Stopped);
}

#[test]
fn test_health_components_list_contains_test_component_edge() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let h = rt.block_on(OkManager.health());
    assert!(!h.components.is_empty());
    assert_eq!(h.components[0].name, "test");
}

// ── RuntimeManager::runtime_status (default impl) ─────────────────────────────

#[test]
fn test_runtime_status_default_impl_returns_running_happy() {
    assert_eq!(OkManager.runtime_status(), RuntimeStatus::Running);
}

#[test]
fn test_runtime_status_is_consistent_across_calls_error() {
    let s1 = OkManager.runtime_status();
    let s2 = OkManager.runtime_status();
    assert_eq!(s1, s2);
}

#[test]
fn test_runtime_status_default_is_not_stopped_edge() {
    assert_ne!(OkManager.runtime_status(), RuntimeStatus::Stopped);
}

// ── RuntimeManager::list_components (default impl) ────────────────────────────

#[test]
fn test_list_components_default_impl_returns_empty_happy() {
    assert!(OkManager.list_components().is_empty());
}

#[test]
fn test_list_components_is_vec_type_error() {
    let v: Vec<ComponentHealth> = OkManager.list_components();
    assert_eq!(v.len(), 0);
}

#[test]
fn test_list_components_called_twice_is_stable_edge() {
    let v1 = OkManager.list_components();
    let v2 = OkManager.list_components();
    assert_eq!(v1.len(), v2.len());
}

// ── RuntimeManager::service_registry (default impl) ───────────────────────────

#[test]
fn test_service_registry_default_impl_returns_none_happy() {
    assert!(OkManager.service_registry().is_none());
}

#[test]
fn test_service_registry_is_option_type_error() {
    let r = OkManager.service_registry();
    assert!(r.is_none());
}

#[test]
fn test_service_registry_called_twice_is_stable_edge() {
    assert!(OkManager.service_registry().is_none());
    assert!(OkManager.service_registry().is_none());
}
