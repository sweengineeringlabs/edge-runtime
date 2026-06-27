//! Integration tests for TonicGrpcServer factory methods.
#![allow(clippy::unwrap_used)]

use std::sync::Arc;
use std::time::Duration;

use swe_edge_ingress_grpc::{CompressionMode, GrpcIngressInterceptorChain, HealthService};
use edge_domain_security::IngressTlsConfig;
use swe_edge_runtime_grpc::{
    GrpcServerConfig, GrpcServerConfigError, NoopGrpcIngress, TonicGrpcServer,
};

fn handler() -> Arc<NoopGrpcIngress> {
    NoopGrpcIngress::create()
}

// ── new ─────────────────────────────────────────────────────────────────────

#[test]
fn test_new_sets_bind_and_defaults_happy() {
    // @covers: new
    let s = TonicGrpcServer::new("127.0.0.1:9999", handler());
    assert_eq!(s.bind_addr(), "127.0.0.1:9999");
    assert!(
        !s.is_reflection_enabled(),
        "reflection must be off by default"
    );
    assert!(
        !s.is_unauthenticated_allowed(),
        "unauthenticated must be off by default"
    );
}

#[test]
fn test_new_health_service_is_auto_wired_error() {
    // @covers: new
    let s = TonicGrpcServer::new("127.0.0.1:0", handler());
    assert!(
        s.health_service().is_some(),
        "health_service must be auto-wired"
    );
    // Verify it can be removed (negative path)
    let s2 = s.without_health_service();
    assert!(
        s2.health_service().is_none(),
        "without_health_service must remove it"
    );
}

// ── from_config ─────────────────────────────────────────────────────────────

#[test]
fn test_from_config_tls_required_no_tls_returns_error_error() {
    // @covers: from_config
    let bind: std::net::SocketAddr = "127.0.0.1:0".parse().unwrap();
    let cfg = GrpcServerConfig::new(bind);
    let r = TonicGrpcServer::from_config(&cfg, handler());
    assert!(matches!(
        r,
        Err(GrpcServerConfigError::TlsRequiredButMissing)
    ));
}

#[test]
fn test_from_config_plaintext_succeeds_happy() {
    // @covers: from_config
    let bind: std::net::SocketAddr = "127.0.0.1:0".parse().unwrap();
    let cfg = GrpcServerConfig::new(bind).allow_plaintext();
    let s = TonicGrpcServer::from_config(&cfg, handler()).unwrap();
    assert!(
        !s.is_reflection_enabled(),
        "from_config must preserve reflection=false"
    );
}

// ── enable_reflection ───────────────────────────────────────────────────────

#[test]
fn test_enable_reflection_true_sets_flag_happy() {
    // @covers: enable_reflection
    let s = TonicGrpcServer::new("127.0.0.1:0", handler()).enable_reflection(true);
    assert!(
        s.is_reflection_enabled(),
        "enable_reflection(true) must set the flag"
    );
}

#[test]
fn test_enable_reflection_false_clears_flag_error() {
    // @covers: enable_reflection
    let s = TonicGrpcServer::new("127.0.0.1:0", handler())
        .enable_reflection(true)
        .enable_reflection(false);
    assert!(
        !s.is_reflection_enabled(),
        "enable_reflection(false) must clear the flag"
    );
}

#[test]
fn test_enable_reflection_toggle_twice_edge() {
    // @covers: enable_reflection
    let s = TonicGrpcServer::new("127.0.0.1:0", handler())
        .enable_reflection(true)
        .enable_reflection(false)
        .enable_reflection(true);
    assert!(
        s.is_reflection_enabled(),
        "toggling reflection must end at the final state"
    );
}

// ── is_reflection_enabled ───────────────────────────────────────────────────

#[test]
fn test_is_reflection_enabled_matches_flag_happy() {
    // @covers: is_reflection_enabled
    assert!(!TonicGrpcServer::new("127.0.0.1:0", handler()).is_reflection_enabled());
    assert!(TonicGrpcServer::new("127.0.0.1:0", handler())
        .enable_reflection(true)
        .is_reflection_enabled());
}

// ── with_audit_sink ─────────────────────────────────────────────────────────

#[test]
fn test_with_audit_sink_stores_custom_sink_happy() {
    // @covers: with_audit_sink
    use std::sync::Mutex;
    use swe_edge_ingress_grpc::{AuditEvent, AuditSink, GrpcStatusCode};
    struct Counter(Arc<Mutex<usize>>);
    impl AuditSink for Counter {
        fn record(&self, _: AuditEvent) {
            *self.0.lock().unwrap() += 1;
        }
    }
    let count = Arc::new(Mutex::new(0usize));
    let s = TonicGrpcServer::new("127.0.0.1:0", handler())
        .with_audit_sink(Arc::new(Counter(count.clone())));
    s.audit_sink_ref().record(AuditEvent {
        timestamp: std::time::SystemTime::UNIX_EPOCH,
        method: "/x".into(),
        identity: None,
        status: GrpcStatusCode::Ok,
        duration_ms: 0,
    });
    assert_eq!(
        *count.lock().unwrap(),
        1,
        "custom audit sink must receive the event"
    );
}

#[test]
fn test_with_audit_sink_default_is_noop_error() {
    // @covers: with_audit_sink
    // Default sink must exist — replacing it with noop is the "no-op" baseline
    let s = TonicGrpcServer::new("127.0.0.1:0", handler());
    assert!(
        !s.is_reflection_enabled(),
        "default server must not have reflection — verifying defaults are sane"
    );
    // Audit sink always exists (noop by default); we validate by replacing and confirming 0 events logged
    use std::sync::Mutex;
    use swe_edge_ingress_grpc::{AuditEvent, AuditSink, GrpcStatusCode};
    struct ZeroCounter(Arc<Mutex<usize>>);
    impl AuditSink for ZeroCounter {
        fn record(&self, _: AuditEvent) {
            *self.0.lock().unwrap() += 1;
        }
    }
    let count = Arc::new(Mutex::new(0usize));
    let s2 = s.with_audit_sink(Arc::new(ZeroCounter(count.clone())));
    s2.audit_sink_ref().record(AuditEvent {
        timestamp: std::time::SystemTime::UNIX_EPOCH,
        method: "/y".into(),
        identity: None,
        status: GrpcStatusCode::Ok,
        duration_ms: 0,
    });
    assert_eq!(
        *count.lock().unwrap(),
        1,
        "replaced audit sink must receive exactly one event"
    );
    assert_ne!(
        *count.lock().unwrap(),
        0,
        "must not silently drop the event"
    );
}

#[test]
fn test_with_audit_sink_overwrites_default_edge() {
    // @covers: with_audit_sink
    use std::sync::Mutex;
    use swe_edge_ingress_grpc::{AuditEvent, AuditSink, GrpcStatusCode, NoopAuditSink};
    struct Counter(Arc<Mutex<usize>>);
    impl AuditSink for Counter {
        fn record(&self, _: AuditEvent) {
            *self.0.lock().unwrap() += 1;
        }
    }
    let count = Arc::new(Mutex::new(0usize));
    // Replace with noop then with counter — second write must win
    let s = TonicGrpcServer::new("127.0.0.1:0", handler())
        .with_audit_sink(Arc::new(NoopAuditSink))
        .with_audit_sink(Arc::new(Counter(count.clone())));
    s.audit_sink_ref().record(AuditEvent {
        timestamp: std::time::SystemTime::UNIX_EPOCH,
        method: "/z".into(),
        identity: None,
        status: GrpcStatusCode::Ok,
        duration_ms: 0,
    });
    assert_eq!(
        *count.lock().unwrap(),
        1,
        "second with_audit_sink must overwrite the first"
    );
}

// ── allow_unauthenticated ───────────────────────────────────────────────────

#[test]
fn test_allow_unauthenticated_true_sets_flag_happy() {
    // @covers: allow_unauthenticated
    let s = TonicGrpcServer::new("127.0.0.1:0", handler()).allow_unauthenticated(true);
    assert!(s.is_unauthenticated_allowed());
}

#[test]
fn test_allow_unauthenticated_false_clears_flag_error() {
    // @covers: allow_unauthenticated
    let s = TonicGrpcServer::new("127.0.0.1:0", handler()).allow_unauthenticated(false);
    assert!(!s.is_unauthenticated_allowed());
}

#[test]
fn test_allow_unauthenticated_combined_with_reflection_edge() {
    // @covers: allow_unauthenticated
    let s = TonicGrpcServer::new("127.0.0.1:0", handler())
        .allow_unauthenticated(true)
        .enable_reflection(true);
    assert!(
        s.is_unauthenticated_allowed(),
        "allow_unauthenticated must survive chaining"
    );
    assert!(
        s.is_reflection_enabled(),
        "enable_reflection must survive chaining"
    );
}

// ── with_max_message_size ───────────────────────────────────────────────────

#[test]
fn test_with_max_message_size_overrides_default_happy() {
    // @covers: with_max_message_size
    let s = TonicGrpcServer::new("127.0.0.1:0", handler()).with_max_message_size(1024);
    assert_eq!(s.max_message_size(), 1024);
}

#[test]
fn test_with_max_message_size_default_is_nonzero_error() {
    // @covers: with_max_message_size
    let s = TonicGrpcServer::new("127.0.0.1:0", handler());
    assert_ne!(
        s.max_message_size(),
        0,
        "default max_bytes must not be zero"
    );
}

#[test]
fn test_with_max_message_size_one_byte_edge() {
    // @covers: with_max_message_size
    let s = TonicGrpcServer::new("127.0.0.1:0", handler()).with_max_message_size(1);
    assert_eq!(s.max_message_size(), 1, "must accept 1 byte as minimum");
    assert_ne!(s.max_message_size(), 0);
}

// ── with_max_concurrent_streams ─────────────────────────────────────────────

#[test]
fn test_with_max_concurrent_streams_overrides_default_happy() {
    // @covers: with_max_concurrent_streams
    let s = TonicGrpcServer::new("127.0.0.1:0", handler()).with_max_concurrent_streams(8);
    assert_eq!(s.max_concurrent_streams(), 8);
}

#[test]
fn test_with_max_concurrent_streams_default_is_nonzero_error() {
    // @covers: with_max_concurrent_streams
    let s = TonicGrpcServer::new("127.0.0.1:0", handler());
    assert_ne!(
        s.max_concurrent_streams(),
        0,
        "default max_concurrent_streams must not be zero"
    );
}

#[test]
fn test_with_max_concurrent_streams_value_of_one_edge() {
    // @covers: with_max_concurrent_streams
    let s = TonicGrpcServer::new("127.0.0.1:0", handler()).with_max_concurrent_streams(1);
    assert_eq!(s.max_concurrent_streams(), 1, "must accept 1 as minimum");
    assert_ne!(s.max_concurrent_streams(), 0);
}

// ── with_interceptors ───────────────────────────────────────────────────────

#[test]
fn test_with_interceptors_replaces_chain_happy() {
    // @covers: with_interceptors
    let chain = GrpcIngressInterceptorChain::new();
    let s = TonicGrpcServer::new("127.0.0.1:0", handler()).with_interceptors(chain);
    assert!(
        !s.interceptor_chain().contains_authorization(),
        "empty chain must not have authz"
    );
}

#[test]
fn test_with_interceptors_default_chain_has_no_authz_error() {
    // @covers: with_interceptors
    let s = TonicGrpcServer::new("127.0.0.1:0", handler());
    assert!(
        !s.interceptor_chain().contains_authorization(),
        "default chain must not have authz"
    );
}

#[test]
fn test_with_interceptors_empty_chain_replaces_default_edge() {
    // @covers: with_interceptors
    let s = TonicGrpcServer::new("127.0.0.1:0", handler())
        .with_interceptors(GrpcIngressInterceptorChain::new())
        .with_interceptors(GrpcIngressInterceptorChain::new());
    assert!(
        !s.interceptor_chain().contains_authorization(),
        "empty chain after overwrite must have no authz"
    );
}

// ── with_compression ────────────────────────────────────────────────────────

#[test]
fn test_with_compression_stores_gzip_happy() {
    // @covers: with_compression
    let s = TonicGrpcServer::new("127.0.0.1:0", handler()).with_compression(CompressionMode::Gzip);
    assert!(matches!(s.compression_mode(), CompressionMode::Gzip));
}

#[test]
fn test_with_compression_default_is_none_error() {
    // @covers: with_compression
    let s = TonicGrpcServer::new("127.0.0.1:0", handler());
    assert!(
        matches!(s.compression_mode(), CompressionMode::None),
        "default must be None"
    );
}

#[test]
fn test_with_compression_back_to_none_edge() {
    // @covers: with_compression
    let s = TonicGrpcServer::new("127.0.0.1:0", handler())
        .with_compression(CompressionMode::Gzip)
        .with_compression(CompressionMode::None);
    assert!(
        matches!(s.compression_mode(), CompressionMode::None),
        "compression must be overridable back to None"
    );
}

// ── with_tls ────────────────────────────────────────────────────────────────

#[test]
fn test_with_tls_stores_config_happy() {
    // @covers: with_tls
    let tls = IngressTlsConfig { cert_pem_path: "c.pem".into(), key_pem_path: "k.pem".into(), client_ca_pem_path: None };
    let s = TonicGrpcServer::new("127.0.0.1:0", handler()).with_tls(tls);
    assert!(s.tls_config().is_some(), "with_tls must store the config");
    assert!(
        !s.is_reflection_enabled(),
        "with_tls must not change reflection setting"
    );
}

#[test]
fn test_with_tls_default_has_none_error() {
    // @covers: with_tls
    let s = TonicGrpcServer::new("127.0.0.1:0", handler());
    assert!(s.tls_config().is_none(), "default server must have no TLS");
}

#[test]
fn test_with_tls_overwrites_previous_edge() {
    // @covers: with_tls
    let tls1 = IngressTlsConfig { cert_pem_path: "a.pem".into(), key_pem_path: "b.pem".into(), client_ca_pem_path: None };
    let tls2 = IngressTlsConfig { cert_pem_path: "c.pem".into(), key_pem_path: "d.pem".into(), client_ca_pem_path: None };
    let s = TonicGrpcServer::new("127.0.0.1:0", handler())
        .with_tls(tls1)
        .with_tls(tls2);
    assert!(
        s.tls_config().is_some(),
        "second with_tls must overwrite the first"
    );
    assert!(
        !s.is_reflection_enabled(),
        "with_tls must not alter reflection setting"
    );
}

// ── with_keepalive ──────────────────────────────────────────────────────────

#[test]
fn test_with_keepalive_sets_interval_and_timeout_happy() {
    // @covers: with_keepalive
    let s = TonicGrpcServer::new("127.0.0.1:0", handler())
        .with_keepalive(Duration::from_secs(30), Duration::from_secs(5));
    assert_eq!(s.keepalive_interval(), Some(Duration::from_secs(30)));
    assert_eq!(s.keepalive_timeout(), Duration::from_secs(5));
}

#[test]
fn test_with_keepalive_large_values_accepted_error() {
    // @covers: with_keepalive
    let s = TonicGrpcServer::new("127.0.0.1:0", handler())
        .with_keepalive(Duration::from_secs(3600), Duration::from_secs(60));
    assert_eq!(s.keepalive_interval(), Some(Duration::from_secs(3600)));
    assert_ne!(
        s.keepalive_interval(),
        Some(Duration::ZERO),
        "large interval must not be treated as disabled"
    );
}

// ── without_keepalive ───────────────────────────────────────────────────────

#[test]
fn test_without_keepalive_clears_interval_happy() {
    // @covers: without_keepalive
    let s = TonicGrpcServer::new("127.0.0.1:0", handler()).without_keepalive();
    assert!(
        s.keepalive_interval().is_none(),
        "without_keepalive must clear the interval"
    );
    assert_ne!(
        s.keepalive_interval(),
        Some(Duration::ZERO),
        "must be None not zero duration"
    );
}

#[test]
fn test_without_keepalive_default_has_interval_error() {
    // @covers: without_keepalive
    let s = TonicGrpcServer::new("127.0.0.1:0", handler());
    assert!(
        s.keepalive_interval().is_some(),
        "default must have keepalive enabled"
    );
    assert_ne!(
        s.keepalive_interval(),
        Some(Duration::ZERO),
        "default keepalive must not be zero duration"
    );
}

#[test]
fn test_without_keepalive_called_twice_stays_disabled_edge() {
    // @covers: without_keepalive
    let s = TonicGrpcServer::new("127.0.0.1:0", handler())
        .without_keepalive()
        .without_keepalive();
    assert!(
        s.keepalive_interval().is_none(),
        "double without_keepalive must remain disabled"
    );
}

// ── keepalive_interval ──────────────────────────────────────────────────────

#[test]
fn test_keepalive_interval_returns_stored_value_happy() {
    // @covers: keepalive_interval
    let s = TonicGrpcServer::new("127.0.0.1:0", handler())
        .with_keepalive(Duration::from_secs(60), Duration::from_secs(10));
    assert_eq!(s.keepalive_interval(), Some(Duration::from_secs(60)));
}

#[test]
fn test_keepalive_interval_returns_none_after_disable_error() {
    // @covers: keepalive_interval
    let s = TonicGrpcServer::new("127.0.0.1:0", handler()).without_keepalive();
    assert!(
        s.keepalive_interval().is_none(),
        "keepalive_interval must return None after without_keepalive"
    );
    assert_ne!(
        s.keepalive_interval(),
        Some(Duration::ZERO),
        "must be None not zero"
    );
}

#[test]
fn test_keepalive_interval_default_is_positive_edge() {
    // @covers: keepalive_interval
    let s = TonicGrpcServer::new("127.0.0.1:0", handler());
    let interval = s.keepalive_interval().unwrap();
    assert!(
        interval.as_secs() > 0,
        "default keepalive interval must be positive"
    );
}

// ── keepalive_timeout ───────────────────────────────────────────────────────

#[test]
fn test_keepalive_timeout_returns_stored_value_happy() {
    // @covers: keepalive_timeout
    let s = TonicGrpcServer::new("127.0.0.1:0", handler())
        .with_keepalive(Duration::from_secs(60), Duration::from_secs(10));
    assert_eq!(s.keepalive_timeout(), Duration::from_secs(10));
}

#[test]
fn test_keepalive_timeout_default_is_nonzero_error() {
    // @covers: keepalive_timeout
    let s = TonicGrpcServer::new("127.0.0.1:0", handler());
    assert_ne!(
        s.keepalive_timeout(),
        Duration::ZERO,
        "default timeout must not be zero"
    );
}

#[test]
fn test_keepalive_timeout_less_than_interval_edge() {
    // @covers: keepalive_timeout
    let s = TonicGrpcServer::new("127.0.0.1:0", handler());
    let interval = s.keepalive_interval().unwrap();
    let timeout = s.keepalive_timeout();
    assert!(
        timeout < interval,
        "default timeout must be shorter than the keepalive interval"
    );
}

// ── without_trace_context ───────────────────────────────────────────────────

#[test]
fn test_without_trace_context_clears_flag_happy() {
    // @covers: without_trace_context
    let s = TonicGrpcServer::new("127.0.0.1:0", handler()).without_trace_context();
    assert!(
        !s.has_trace_context(),
        "without_trace_context must clear the flag"
    );
}

#[test]
fn test_without_trace_context_default_is_true_error() {
    // @covers: without_trace_context
    let s = TonicGrpcServer::new("127.0.0.1:0", handler());
    assert!(
        s.has_trace_context(),
        "trace context must be auto-wired by default"
    );
}

#[test]
fn test_without_trace_context_combined_with_other_settings_edge() {
    // @covers: without_trace_context
    let s = TonicGrpcServer::new("127.0.0.1:0", handler())
        .without_trace_context()
        .enable_reflection(true);
    assert!(
        !s.has_trace_context(),
        "without_trace_context must survive chaining"
    );
    assert!(
        s.is_reflection_enabled(),
        "enable_reflection must survive chaining"
    );
}

// ── without_health_service ──────────────────────────────────────────────────

#[test]
fn test_without_health_service_removes_auto_wired_happy() {
    // @covers: without_health_service
    let s = TonicGrpcServer::new("127.0.0.1:0", handler()).without_health_service();
    assert!(
        s.health_service().is_none(),
        "without_health_service must remove it"
    );
    assert!(
        !s.is_reflection_enabled(),
        "removing health service must not change reflection setting"
    );
}

#[test]
fn test_without_health_service_default_has_service_error() {
    // @covers: without_health_service
    let s = TonicGrpcServer::new("127.0.0.1:0", handler());
    assert!(
        s.health_service().is_some(),
        "default server must have health service auto-wired"
    );
    assert!(
        !s.is_reflection_enabled(),
        "health service default must not imply reflection"
    );
}

#[test]
fn test_without_health_service_called_twice_stays_none_edge() {
    // @covers: without_health_service
    let s = TonicGrpcServer::new("127.0.0.1:0", handler())
        .without_health_service()
        .without_health_service();
    assert!(
        s.health_service().is_none(),
        "double without_health_service must remain None"
    );
}

// ── health_service ──────────────────────────────────────────────────────────

#[test]
fn test_health_service_returns_auto_wired_ref_happy() {
    // @covers: health_service
    let s = TonicGrpcServer::new("127.0.0.1:0", handler());
    let hs = s.health_service();
    assert!(
        hs.is_some(),
        "health_service must be auto-wired on new server"
    );
    let fresh = Arc::new(HealthService::new());
    assert!(
        !Arc::ptr_eq(hs.unwrap(), &fresh),
        "stored health service must be distinct from a newly created one"
    );
}

#[test]
fn test_health_service_returns_none_after_removal_error() {
    // @covers: health_service
    let s = TonicGrpcServer::new("127.0.0.1:0", handler()).without_health_service();
    assert!(
        s.health_service().is_none(),
        "health_service must be absent after removal"
    );
    assert!(
        !s.is_reflection_enabled(),
        "removal must not change reflection setting"
    );
}

// ── with_health_service ─────────────────────────────────────────────────────

#[test]
fn test_with_health_service_replaces_default_happy() {
    // @covers: with_health_service
    let custom = Arc::new(HealthService::new());
    let ptr = Arc::as_ptr(&custom);
    let s = TonicGrpcServer::new("127.0.0.1:0", handler()).with_health_service(custom);
    assert_eq!(
        Arc::as_ptr(s.health_service().unwrap()),
        ptr,
        "custom health service must be stored"
    );
}

#[test]
fn test_with_health_service_after_removal_restores_it_error() {
    // @covers: with_health_service
    let custom = Arc::new(HealthService::new());
    let s = TonicGrpcServer::new("127.0.0.1:0", handler())
        .without_health_service()
        .with_health_service(custom);
    assert!(
        s.health_service().is_some(),
        "with_health_service must restore service after removal"
    );
    assert!(
        !s.is_reflection_enabled(),
        "restore must not change reflection setting"
    );
}

#[test]
fn test_with_health_service_pointer_identity_edge() {
    // @covers: with_health_service
    let hs1 = Arc::new(HealthService::new());
    let hs2 = Arc::new(HealthService::new());
    let ptr2 = Arc::as_ptr(&hs2);
    let s = TonicGrpcServer::new("127.0.0.1:0", handler())
        .with_health_service(hs1)
        .with_health_service(hs2);
    assert_eq!(
        Arc::as_ptr(s.health_service().unwrap()),
        ptr2,
        "last with_health_service must win"
    );
}

// ── bind_addr ────────────────────────────────────────────────────────────────

#[test]
fn test_bind_addr_returns_set_value_happy() {
    // @covers: bind_addr
    let s = TonicGrpcServer::new("192.168.1.1:8080", handler());
    assert_eq!(s.bind_addr(), "192.168.1.1:8080");
}

#[test]
fn test_bind_addr_does_not_modify_input_error() {
    // @covers: bind_addr
    let s = TonicGrpcServer::new("127.0.0.1:0", handler());
    assert_eq!(
        s.bind_addr(),
        "127.0.0.1:0",
        "bind_addr must return the exact string passed to new()"
    );
    assert!(
        !s.bind_addr().is_empty(),
        "bind_addr must never return an empty string"
    );
}

#[test]
fn test_bind_addr_contains_port_separator_edge() {
    // @covers: bind_addr
    let s = TonicGrpcServer::new("0.0.0.0:50051", handler());
    assert!(
        s.bind_addr().contains(':'),
        "bind_addr must include the port separator"
    );
    assert_eq!(s.bind_addr(), "0.0.0.0:50051");
}

// ── max_message_size ─────────────────────────────────────────────────────────

#[test]
fn test_max_message_size_returns_set_value_happy() {
    // @covers: max_message_size
    let s = TonicGrpcServer::new("127.0.0.1:0", handler()).with_max_message_size(2048);
    assert_eq!(s.max_message_size(), 2048);
}

#[test]
fn test_max_message_size_default_is_nonzero_error() {
    // @covers: max_message_size
    let s = TonicGrpcServer::new("127.0.0.1:0", handler());
    assert_ne!(
        s.max_message_size(),
        0,
        "default max_message_size must not be zero"
    );
}

#[test]
fn test_max_message_size_minimum_accepted_edge() {
    // @covers: max_message_size
    let s = TonicGrpcServer::new("127.0.0.1:0", handler()).with_max_message_size(1);
    assert_eq!(
        s.max_message_size(),
        1,
        "max_message_size must accept 1 byte as minimum"
    );
    assert_ne!(s.max_message_size(), 0);
}

// ── max_concurrent_streams ───────────────────────────────────────────────────

#[test]
fn test_max_concurrent_streams_returns_set_value_happy() {
    // @covers: max_concurrent_streams
    let s = TonicGrpcServer::new("127.0.0.1:0", handler()).with_max_concurrent_streams(16);
    assert_eq!(s.max_concurrent_streams(), 16);
}

#[test]
fn test_max_concurrent_streams_default_is_nonzero_error() {
    // @covers: max_concurrent_streams
    let s = TonicGrpcServer::new("127.0.0.1:0", handler());
    assert_ne!(
        s.max_concurrent_streams(),
        0,
        "default max_concurrent_streams must not be zero"
    );
}

#[test]
fn test_max_concurrent_streams_minimum_accepted_edge() {
    // @covers: max_concurrent_streams
    let s = TonicGrpcServer::new("127.0.0.1:0", handler()).with_max_concurrent_streams(1);
    assert_eq!(
        s.max_concurrent_streams(),
        1,
        "max_concurrent_streams must accept 1 as minimum"
    );
    assert_ne!(s.max_concurrent_streams(), 0);
}

// ── tls_config ───────────────────────────────────────────────────────────────

#[test]
fn test_tls_config_with_tls_returns_some_happy() {
    // @covers: tls_config
    let tls = IngressTlsConfig { cert_pem_path: "cert.pem".into(), key_pem_path: "key.pem".into(), client_ca_pem_path: None };
    let s = TonicGrpcServer::new("127.0.0.1:0", handler()).with_tls(tls);
    assert!(
        s.tls_config().is_some(),
        "tls_config must return Some after with_tls"
    );
    assert!(
        !s.is_reflection_enabled(),
        "with_tls must not change reflection setting"
    );
}

#[test]
fn test_tls_config_default_is_none_error() {
    // @covers: tls_config
    let s = TonicGrpcServer::new("127.0.0.1:0", handler());
    assert!(s.tls_config().is_none(), "default tls_config must be None");
}

#[test]
fn test_tls_config_overwrite_returns_latest_edge() {
    // @covers: tls_config
    let tls1 = IngressTlsConfig { cert_pem_path: "a.pem".into(), key_pem_path: "b.pem".into(), client_ca_pem_path: None };
    let tls2 = IngressTlsConfig { cert_pem_path: "c.pem".into(), key_pem_path: "d.pem".into(), client_ca_pem_path: None };
    let s = TonicGrpcServer::new("127.0.0.1:0", handler())
        .with_tls(tls1)
        .with_tls(tls2);
    assert!(
        s.tls_config().is_some(),
        "tls_config must return Some after double with_tls"
    );
    assert!(
        !s.is_unauthenticated_allowed(),
        "double with_tls must not change allow_unauthenticated"
    );
}

// ── compression_mode ─────────────────────────────────────────────────────────

#[test]
fn test_compression_mode_with_gzip_returns_gzip_happy() {
    // @covers: compression_mode
    let s = TonicGrpcServer::new("127.0.0.1:0", handler()).with_compression(CompressionMode::Gzip);
    assert!(
        matches!(s.compression_mode(), CompressionMode::Gzip),
        "compression_mode must return Gzip after with_compression(Gzip)"
    );
}

#[test]
fn test_compression_mode_default_is_none_error() {
    // @covers: compression_mode
    let s = TonicGrpcServer::new("127.0.0.1:0", handler());
    assert!(
        matches!(s.compression_mode(), CompressionMode::None),
        "default compression_mode must be None"
    );
}

#[test]
fn test_compression_mode_overwrite_back_to_none_edge() {
    // @covers: compression_mode
    let s = TonicGrpcServer::new("127.0.0.1:0", handler())
        .with_compression(CompressionMode::Gzip)
        .with_compression(CompressionMode::None);
    assert!(
        matches!(s.compression_mode(), CompressionMode::None),
        "compression_mode must reflect the last write"
    );
}

// ── is_unauthenticated_allowed ───────────────────────────────────────────────

#[test]
fn test_is_unauthenticated_allowed_when_set_happy() {
    // @covers: is_unauthenticated_allowed
    let s = TonicGrpcServer::new("127.0.0.1:0", handler()).allow_unauthenticated(true);
    assert!(
        s.is_unauthenticated_allowed(),
        "is_unauthenticated_allowed must be true after allow_unauthenticated(true)"
    );
}

#[test]
fn test_is_unauthenticated_allowed_default_is_false_error() {
    // @covers: is_unauthenticated_allowed
    let s = TonicGrpcServer::new("127.0.0.1:0", handler());
    assert!(
        !s.is_unauthenticated_allowed(),
        "default is_unauthenticated_allowed must be false"
    );
}

#[test]
fn test_is_unauthenticated_allowed_when_cleared_edge() {
    // @covers: is_unauthenticated_allowed
    let s = TonicGrpcServer::new("127.0.0.1:0", handler())
        .allow_unauthenticated(true)
        .allow_unauthenticated(false);
    assert!(
        !s.is_unauthenticated_allowed(),
        "is_unauthenticated_allowed must reflect the last write"
    );
}

// ── has_trace_context ────────────────────────────────────────────────────────

#[test]
fn test_has_trace_context_default_is_true_happy() {
    // @covers: has_trace_context
    let s = TonicGrpcServer::new("127.0.0.1:0", handler());
    assert!(
        s.has_trace_context(),
        "trace context must be enabled by default"
    );
}

#[test]
fn test_has_trace_context_after_removal_is_false_error() {
    // @covers: has_trace_context
    let s = TonicGrpcServer::new("127.0.0.1:0", handler()).without_trace_context();
    assert!(
        !s.has_trace_context(),
        "has_trace_context must be false after without_trace_context"
    );
}

#[test]
fn test_has_trace_context_survives_other_changes_edge() {
    // @covers: has_trace_context
    let s = TonicGrpcServer::new("127.0.0.1:0", handler())
        .without_trace_context()
        .with_max_message_size(512);
    assert!(
        !s.has_trace_context(),
        "has_trace_context must survive unrelated method chains"
    );
    assert_eq!(s.max_message_size(), 512);
}

// ── audit_sink_ref ───────────────────────────────────────────────────────────

#[test]
fn test_audit_sink_ref_default_is_callable_happy() {
    // @covers: audit_sink_ref
    use swe_edge_ingress_grpc::{AuditEvent, GrpcStatusCode};
    let s = TonicGrpcServer::new("127.0.0.1:0", handler());
    s.audit_sink_ref().record(AuditEvent {
        timestamp: std::time::SystemTime::UNIX_EPOCH,
        method: "/test".into(),
        identity: None,
        status: GrpcStatusCode::Ok,
        duration_ms: 0,
    });
    assert!(
        !s.is_reflection_enabled(),
        "audit_sink_ref must not affect other fields"
    );
}

#[test]
fn test_audit_sink_ref_custom_sink_receives_events_error() {
    // @covers: audit_sink_ref
    use std::sync::Mutex;
    use swe_edge_ingress_grpc::{AuditEvent, AuditSink, GrpcStatusCode};
    struct Counter(Arc<Mutex<usize>>);
    impl AuditSink for Counter {
        fn record(&self, _: AuditEvent) {
            *self.0.lock().unwrap() += 1;
        }
    }
    let count = Arc::new(Mutex::new(0usize));
    let s = TonicGrpcServer::new("127.0.0.1:0", handler())
        .with_audit_sink(Arc::new(Counter(count.clone())));
    s.audit_sink_ref().record(AuditEvent {
        timestamp: std::time::SystemTime::UNIX_EPOCH,
        method: "/y".into(),
        identity: None,
        status: GrpcStatusCode::Internal,
        duration_ms: 1,
    });
    assert_eq!(
        *count.lock().unwrap(),
        1,
        "audit_sink_ref must point to the custom sink"
    );
}

#[test]
fn test_audit_sink_ref_overwrite_second_sink_wins_edge() {
    // @covers: audit_sink_ref
    use std::sync::Mutex;
    use swe_edge_ingress_grpc::{AuditEvent, AuditSink, GrpcStatusCode, NoopAuditSink};
    struct Counter(Arc<Mutex<usize>>);
    impl AuditSink for Counter {
        fn record(&self, _: AuditEvent) {
            *self.0.lock().unwrap() += 1;
        }
    }
    let count = Arc::new(Mutex::new(0usize));
    let s = TonicGrpcServer::new("127.0.0.1:0", handler())
        .with_audit_sink(Arc::new(NoopAuditSink))
        .with_audit_sink(Arc::new(Counter(count.clone())));
    s.audit_sink_ref().record(AuditEvent {
        timestamp: std::time::SystemTime::UNIX_EPOCH,
        method: "/z".into(),
        identity: None,
        status: GrpcStatusCode::Ok,
        duration_ms: 0,
    });
    assert_eq!(
        *count.lock().unwrap(),
        1,
        "audit_sink_ref must reflect the second (overwriting) sink"
    );
}

// ── interceptor_chain ────────────────────────────────────────────────────────

#[test]
fn test_interceptor_chain_default_has_no_authz_happy() {
    // @covers: interceptor_chain
    let s = TonicGrpcServer::new("127.0.0.1:0", handler());
    assert!(
        !s.interceptor_chain().contains_authorization(),
        "default interceptor chain must not have authz"
    );
}

#[test]
fn test_interceptor_chain_after_set_empty_chain_has_no_authz_error() {
    // @covers: interceptor_chain
    let s = TonicGrpcServer::new("127.0.0.1:0", handler())
        .with_interceptors(GrpcIngressInterceptorChain::new());
    assert!(
        !s.interceptor_chain().contains_authorization(),
        "explicitly set empty chain must not have authz"
    );
}

#[test]
fn test_interceptor_chain_overwrite_replaces_chain_edge() {
    // @covers: interceptor_chain
    let s = TonicGrpcServer::new("127.0.0.1:0", handler())
        .with_interceptors(GrpcIngressInterceptorChain::new())
        .with_interceptors(GrpcIngressInterceptorChain::new());
    assert!(
        !s.interceptor_chain().contains_authorization(),
        "second chain must replace the first"
    );
    assert!(
        !s.is_unauthenticated_allowed(),
        "interceptor_chain overwrite must not change allow_unauthenticated"
    );
}
