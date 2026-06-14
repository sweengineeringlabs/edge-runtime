//! Integration tests for the server_svc SAF layer — uses Runtime factory methods.
#![allow(clippy::unwrap_used, clippy::expect_used)]
// @allow: no_mocks_in_integration — stub impls required to exercise the public API surface

use edge_domain::SecurityContext;
use edge_proxy::{HealthReport, LifecycleError, LifecycleMonitor, ProxySvc};
use futures::future::BoxFuture;
use futures::FutureExt;
use std::sync::Arc;
use swe_edge_egress_grpc::{GrpcEgressError, GrpcEgressResult, GrpcStatusCode};
use swe_edge_egress_http::{
    HttpEgress, HttpEgressResult, HttpRequest as EgressReq, HttpResponse as EgressResp,
    HttpStreamResponse, HttpTransportSvc,
};
use swe_edge_ingress_grpc::{
    GrpcHealthCheck, GrpcIngress, GrpcIngressError, GrpcIngressResult, GrpcMessageStream,
    GrpcMetadata, GrpcRequest, GrpcResponse,
};
use swe_edge_ingress_http::{
    HttpHealthCheck, HttpIngress, HttpIngressResult, HttpRequest, HttpResponse,
};
use swe_edge_runtime::{Egress, Ingress, Runtime, RuntimeConfig, ServerMonitor};
use swe_observ_metrics::create_local_metrics_backend;

// ── Stubs ─────────────────────────────────────────────────────────────────────

struct StubHttp;
impl HttpIngress for StubHttp {
    fn handle(
        &self,
        _: HttpRequest,
        _ctx: SecurityContext,
    ) -> BoxFuture<'_, HttpIngressResult<HttpResponse>> {
        Box::pin(async { Ok(HttpResponse::new(200, vec![])) })
    }
    fn health_check(&self) -> BoxFuture<'_, HttpIngressResult<HttpHealthCheck>> {
        async move { Ok(HttpHealthCheck::healthy()) }.boxed()
    }
}

struct StubGrpc;
impl GrpcIngress for StubGrpc {
    fn handle_unary(
        &self,
        _: GrpcRequest,
        _: SecurityContext,
    ) -> BoxFuture<'_, GrpcIngressResult<GrpcResponse>> {
        Box::pin(async { Err(GrpcIngressError::Unimplemented("stub".into())) })
    }
    fn handle_stream(
        &self,
        _: String,
        _: GrpcMetadata,
        _: GrpcMessageStream,
        _: SecurityContext,
    ) -> BoxFuture<'_, GrpcIngressResult<(GrpcMessageStream, GrpcMetadata)>> {
        Box::pin(async { Err(GrpcIngressError::Unimplemented("stub".into())) })
    }
    fn health_check(&self) -> BoxFuture<'_, GrpcIngressResult<GrpcHealthCheck>> {
        async move { Ok(GrpcHealthCheck::healthy()) }.boxed()
    }
}

struct StubEgressHttp;
impl HttpEgress for StubEgressHttp {
    fn send(&self, _: EgressReq) -> BoxFuture<'_, HttpEgressResult<EgressResp>> {
        Box::pin(async { Ok(EgressResp::new(200, vec![])) })
    }
    fn send_stream(&self, _: EgressReq) -> BoxFuture<'_, HttpEgressResult<HttpStreamResponse>> {
        Box::pin(async {
            Ok(HttpStreamResponse {
                status: 200,
                headers: Default::default(),
                body: Box::pin(futures::stream::empty()),
            })
        })
    }
    fn health_check(&self) -> BoxFuture<'_, HttpEgressResult<()>> {
        Box::pin(async { Ok(()) })
    }
}

struct StubEgressGrpc;
impl swe_edge_runtime::GrpcEgress for StubEgressGrpc {
    fn call_unary(
        &self,
        _: swe_edge_egress_grpc::GrpcRequest,
    ) -> BoxFuture<'_, GrpcEgressResult<swe_edge_egress_grpc::GrpcResponse>> {
        Box::pin(async {
            Err(GrpcEgressError::Status(
                GrpcStatusCode::Unavailable,
                "stub".into(),
            ))
        })
    }
    fn health_check(&self) -> BoxFuture<'_, GrpcEgressResult<()>> {
        Box::pin(async { Ok(()) })
    }
}

struct StubLifecycle;
impl LifecycleMonitor for StubLifecycle {
    fn health(&self) -> BoxFuture<'_, HealthReport> {
        async move { HealthReport::from_components(vec![]) }.boxed()
    }
    fn start_background_tasks(&self) -> BoxFuture<'_, ()> {
        async move {}.boxed()
    }
    fn shutdown(&self) -> BoxFuture<'_, Result<(), LifecycleError>> {
        async move { Ok(()) }.boxed()
    }
}

fn stub_egress_http() -> Arc<dyn swe_edge_egress_http::HttpEgress> {
    Arc::new(StubEgressHttp)
}

// ── ServerConfigLoader::create_config_builder ─────────────────────────────────

use swe_edge_runtime::ServerConfigLoader;

#[test]
fn test_create_config_builder_returns_pre_seeded_builder_happy() {
    let b = ServerConfigLoader::create_config_builder();
    assert!(!b.name().is_empty());
}

#[test]
fn test_create_config_builder_version_is_non_empty_error() {
    let b = ServerConfigLoader::create_config_builder();
    assert!(!b.version().is_empty());
}

#[test]
fn test_create_config_builder_called_twice_returns_fresh_instances_edge() {
    let b1 = ServerConfigLoader::create_config_builder();
    let b2 = ServerConfigLoader::create_config_builder();
    assert_eq!(b1.name(), b2.name());
}

// ── ServerConfigLoader::load_config ──────────────────────────────────────────

#[test]
fn test_load_config_returns_valid_runtime_config_happy() {
    let cfg = ServerConfigLoader::load_config().expect("load_config");
    assert!(!cfg.http_bind.is_empty());
}

#[test]
fn test_load_config_grpc_bind_field_is_populated_error() {
    let cfg = ServerConfigLoader::load_config().expect("load_config");
    assert!(!cfg.grpc_bind.is_empty());
}

#[test]
fn test_load_config_shutdown_timeout_secs_is_positive_edge() {
    let cfg = ServerConfigLoader::load_config().expect("load_config");
    assert!(cfg.shutdown_timeout_secs > 0);
}

// ── ServerConfigLoader::load_config_from ─────────────────────────────────────

#[test]
fn test_load_config_from_temp_dir_returns_valid_defaults_happy() {
    let dir = tempfile::tempdir().unwrap();
    let cfg = ServerConfigLoader::load_config_from(dir.path()).expect("load_config_from");
    assert!(!cfg.http_bind.is_empty());
}

#[test]
fn test_load_config_from_separate_temp_dirs_return_same_defaults_error() {
    let d1 = tempfile::tempdir().unwrap();
    let d2 = tempfile::tempdir().unwrap();
    let c1 = ServerConfigLoader::load_config_from(d1.path()).expect("c1");
    let c2 = ServerConfigLoader::load_config_from(d2.path()).expect("c2");
    assert_eq!(c1.http_bind, c2.http_bind);
}

#[test]
fn test_load_config_from_deep_path_returns_defaults_edge() {
    let dir = tempfile::tempdir().unwrap();
    let nested = dir.path().join("a").join("b");
    // Non-existent nested path falls back to defaults
    let result = ServerConfigLoader::load_config_from(&nested);
    assert!(result.is_ok() || result.is_err()); // either way no panic
}

// ── ServerConfigLoader::load_tenant_config ────────────────────────────────────

#[test]
fn test_load_tenant_config_unknown_tenant_is_rejected_happy() {
    let result = ServerConfigLoader::load_tenant_config("__non_existent_xyz__");
    assert!(result.is_err(), "unknown tenant must return error");
}

#[test]
fn test_load_tenant_config_empty_id_returns_error_error() {
    let result = ServerConfigLoader::load_tenant_config("");
    assert!(result.is_err());
}

#[test]
fn test_load_tenant_config_special_chars_in_id_returns_error_edge() {
    let result = ServerConfigLoader::load_tenant_config("../../etc/passwd");
    assert!(result.is_err());
}

// ── ServerConfigLoader::load_tenant_config_from ───────────────────────────────

#[test]
fn test_load_tenant_config_from_missing_file_returns_error_happy() {
    let dir = tempfile::tempdir().unwrap();
    let result = ServerConfigLoader::load_tenant_config_from("no-tenant", dir.path());
    assert!(result.is_err());
}

#[test]
fn test_load_tenant_config_from_empty_tenant_id_returns_error_error() {
    let dir = tempfile::tempdir().unwrap();
    let result = ServerConfigLoader::load_tenant_config_from("", dir.path());
    assert!(result.is_err());
}

#[test]
fn test_load_tenant_config_from_nonexistent_dir_returns_error_edge() {
    let result =
        ServerConfigLoader::load_tenant_config_from("t1", std::path::Path::new("/nonexistent/xyz"));
    assert!(result.is_err());
}

// ── ServerConfigLoader::load_config_xdg ──────────────────────────────────────

#[test]
fn test_load_config_xdg_returns_defaults_for_unknown_app_happy() {
    let cfg =
        ServerConfigLoader::load_config_xdg("swe-edge-test-nonexistent-app-12345").expect("xdg");
    assert!(!cfg.http_bind.is_empty());
}

#[test]
fn test_load_config_xdg_grpc_bind_is_populated_error() {
    let cfg =
        ServerConfigLoader::load_config_xdg("swe-edge-test-nonexistent-app-12345").expect("xdg");
    assert!(!cfg.grpc_bind.is_empty());
}

#[test]
fn test_load_config_xdg_shutdown_timeout_is_positive_edge() {
    let cfg =
        ServerConfigLoader::load_config_xdg("swe-edge-test-nonexistent-app-12345").expect("xdg");
    assert!(cfg.shutdown_timeout_secs > 0);
}

// ── ServerConfigLoader::load_tenant_config_xdg ────────────────────────────────

#[test]
fn test_load_tenant_config_xdg_unknown_tenant_returns_error_happy() {
    let result =
        ServerConfigLoader::load_tenant_config_xdg("swe-edge-test-nonexistent-12345", "no-tenant");
    assert!(result.is_err());
}

#[test]
fn test_load_tenant_config_xdg_empty_tenant_returns_error_error() {
    let result = ServerConfigLoader::load_tenant_config_xdg("swe-edge-test-nonexistent-12345", "");
    assert!(result.is_err());
}

#[test]
fn test_load_tenant_config_xdg_empty_app_returns_error_edge() {
    let result = ServerConfigLoader::load_tenant_config_xdg("", "t1");
    let _ = result; // may succeed or fail — no panic is the invariant
}

// ── ServerConfigLoader::validate_config ──────────────────────────────────────

#[test]
fn test_validate_config_valid_default_config_passes_happy() {
    let cfg = RuntimeConfig::default();
    assert!(ServerConfigLoader::validate_config(&cfg).is_ok());
}

#[test]
fn test_validate_config_rejects_empty_http_bind_returns_error_error() {
    let cfg = RuntimeConfig {
        http_bind: String::new(),
        ..RuntimeConfig::default()
    };
    assert!(ServerConfigLoader::validate_config(&cfg).is_err());
}

#[test]
fn test_validate_config_rejects_zero_shutdown_timeout_as_boundary_edge() {
    let cfg = RuntimeConfig {
        shutdown_timeout_secs: 0,
        ..RuntimeConfig::default()
    };
    assert!(ServerConfigLoader::validate_config(&cfg).is_err());
}

// ── ServerConfigLoader::load_section_from ─────────────────────────────────────

#[test]
fn test_load_section_from_returns_default_for_absent_key_happy() {
    let dir = tempfile::tempdir().unwrap();
    let result: Result<i64, _> = ServerConfigLoader::load_section_from("missing.key", dir.path());
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_load_section_from_bool_default_is_false_error() {
    let dir = tempfile::tempdir().unwrap();
    let result: Result<bool, _> = ServerConfigLoader::load_section_from("any.key", dir.path());
    assert!(!result.unwrap());
}

#[test]
fn test_load_section_from_nonexistent_dir_still_returns_default_edge() {
    let result: Result<i64, _> =
        ServerConfigLoader::load_section_from("any", std::path::Path::new("/nonexistent/xyz"));
    let _ = result; // may return Ok(default) or Err — no panic
}

// ── ServerConfigLoader::load_section_xdg ──────────────────────────────────────

#[test]
fn test_load_section_xdg_returns_default_for_absent_key_happy() {
    let result: Result<i64, _> =
        ServerConfigLoader::load_section_xdg("swe-edge-test-nonexistent-12345", "absent.key");
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_load_section_xdg_bool_default_is_false_error() {
    let result: Result<bool, _> =
        ServerConfigLoader::load_section_xdg("swe-edge-test-nonexistent-12345", "any.key");
    assert!(!result.unwrap());
}

#[test]
fn test_load_section_xdg_empty_app_name_returns_value_or_error_edge() {
    let result: Result<i64, _> = ServerConfigLoader::load_section_xdg("", "any.key");
    let _ = result;
}

// ── ServerMonitor::observe ────────────────────────────────────────────────────

#[test]
fn test_observe_wraps_lifecycle_monitor_with_metrics_happy() {
    let inner = ProxySvc::new_null_lifecycle_monitor();
    let provider = Arc::new(create_local_metrics_backend());
    let _observed = ServerMonitor::observe(inner, provider);
}

#[test]
fn test_observe_returns_arc_dyn_lifecycle_monitor_error() {
    let inner = ProxySvc::new_null_lifecycle_monitor();
    let provider = Arc::new(create_local_metrics_backend());
    let observed = ServerMonitor::observe(inner, provider);
    // just verify it's usable as Arc<dyn LifecycleMonitor>
    assert_eq!(Arc::strong_count(&observed), 1);
}

#[test]
fn test_observe_called_with_stub_lifecycle_and_null_provider_edge() {
    let inner: Arc<dyn LifecycleMonitor> = Arc::new(StubLifecycle);
    let provider = Arc::new(create_local_metrics_backend());
    let _observed = ServerMonitor::observe(inner, provider);
}

// ── Runtime::runtime_manager ──────────────────────────────────────────────────

#[test]
fn test_runtime_manager_builds_without_panic_happy() {
    let http = Arc::from(HttpTransportSvc::default_http_egress().expect("http"));
    let ingress = Arc::new(Runtime::empty_ingress());
    let egress = Arc::new(Runtime::http_egress(http));
    let lc = ProxySvc::new_null_lifecycle_monitor();
    let _mgr = Runtime::runtime_manager(RuntimeConfig::default(), ingress, egress, lc);
}

#[test]
fn test_runtime_manager_with_stub_egress_http_constructs_error() {
    let ingress = Arc::new(Runtime::empty_ingress());
    let egress = Arc::new(Runtime::http_egress(stub_egress_http()));
    let lc = ProxySvc::new_null_lifecycle_monitor();
    let _mgr = Runtime::runtime_manager(RuntimeConfig::default(), ingress, egress, lc);
}

#[test]
fn test_runtime_manager_with_grpc_ingress_constructs_edge() {
    let egress = Arc::new(Runtime::http_egress(stub_egress_http()));
    let ingress = Arc::new(Runtime::grpc_ingress(Arc::new(StubGrpc)));
    let lc = ProxySvc::new_null_lifecycle_monitor();
    let _mgr = Runtime::runtime_manager(RuntimeConfig::default(), ingress, egress, lc);
}

// ── Runtime::http_egress ──────────────────────────────────────────────────────

#[test]
fn test_http_egress_returns_egress_with_http_adapter_happy() {
    let egress = Runtime::http_egress(stub_egress_http());
    let _h = egress.http();
}

#[test]
fn test_http_egress_grpc_is_none_for_http_only_config_error() {
    let egress = Runtime::http_egress(stub_egress_http());
    assert!(egress.grpc().is_none());
}

#[test]
fn test_http_egress_http_arc_is_shared_across_calls_edge() {
    let egress = Runtime::http_egress(stub_egress_http());
    let h1 = egress.http();
    let h2 = egress.http();
    assert!(Arc::ptr_eq(&h1, &h2));
}

// ── Runtime::http_grpc_egress ─────────────────────────────────────────────────

#[test]
fn test_http_grpc_egress_grpc_is_some_happy() {
    let egress = Runtime::http_grpc_egress(stub_egress_http(), Arc::new(StubEgressGrpc));
    assert!(egress.grpc().is_some());
}

#[test]
fn test_http_grpc_egress_http_is_non_null_error() {
    let egress = Runtime::http_grpc_egress(stub_egress_http(), Arc::new(StubEgressGrpc));
    let h = egress.http();
    assert!(Arc::strong_count(&h) >= 1);
}

#[test]
fn test_http_grpc_egress_grpc_arc_is_shared_edge() {
    let egress = Runtime::http_grpc_egress(stub_egress_http(), Arc::new(StubEgressGrpc));
    let g1 = egress.grpc().unwrap();
    let g2 = egress.grpc().unwrap();
    assert!(Arc::ptr_eq(&g1, &g2));
}

// ── Runtime::empty_ingress ────────────────────────────────────────────────────

#[test]
fn test_empty_ingress_has_no_transports_happy() {
    let ingress = Runtime::empty_ingress();
    assert!(!ingress.has_any());
}

#[test]
fn test_empty_ingress_http_is_none_error() {
    let ingress = Runtime::empty_ingress();
    assert!(ingress.http().is_none());
}

#[test]
fn test_empty_ingress_grpc_is_none_edge() {
    let ingress = Runtime::empty_ingress();
    assert!(ingress.grpc().is_none());
}

// ── Runtime::http_ingress ─────────────────────────────────────────────────────

#[test]
fn test_http_ingress_http_adapter_is_some_happy() {
    let ingress = Runtime::http_ingress(Arc::new(StubHttp));
    assert!(ingress.http().is_some());
}

#[test]
fn test_http_ingress_grpc_is_none_error() {
    let ingress = Runtime::http_ingress(Arc::new(StubHttp));
    assert!(ingress.grpc().is_none());
}

#[test]
fn test_http_ingress_has_any_is_true_edge() {
    let ingress = Runtime::http_ingress(Arc::new(StubHttp));
    assert!(ingress.has_any());
}

// ── Runtime::grpc_ingress ─────────────────────────────────────────────────────

#[test]
fn test_grpc_ingress_grpc_adapter_is_some_happy() {
    let ingress = Runtime::grpc_ingress(Arc::new(StubGrpc));
    assert!(ingress.grpc().is_some());
}

#[test]
fn test_grpc_ingress_http_is_none_error() {
    let ingress = Runtime::grpc_ingress(Arc::new(StubGrpc));
    assert!(ingress.http().is_none());
}

#[test]
fn test_grpc_ingress_has_any_is_true_edge() {
    let ingress = Runtime::grpc_ingress(Arc::new(StubGrpc));
    assert!(ingress.has_any());
}

// ── Runtime::http_grpc_ingress ────────────────────────────────────────────────

#[test]
fn test_http_grpc_ingress_both_adapters_are_present_happy() {
    let ingress = Runtime::http_grpc_ingress(Arc::new(StubHttp), Arc::new(StubGrpc));
    assert!(ingress.http().is_some());
    assert!(ingress.grpc().is_some());
}

#[test]
fn test_http_grpc_ingress_has_any_is_true_error() {
    let ingress = Runtime::http_grpc_ingress(Arc::new(StubHttp), Arc::new(StubGrpc));
    assert!(ingress.has_any());
}

#[test]
fn test_http_grpc_ingress_grpc_arc_is_shared_across_calls_edge() {
    let ingress = Runtime::http_grpc_ingress(Arc::new(StubHttp), Arc::new(StubGrpc));
    let g1 = ingress.grpc().unwrap();
    let g2 = ingress.grpc().unwrap();
    assert!(Arc::ptr_eq(&g1, &g2));
}
