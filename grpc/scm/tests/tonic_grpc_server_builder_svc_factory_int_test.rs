//! Integration tests for TonicGrpcServerBuilder factory methods.
#![allow(clippy::unwrap_used)]

use std::sync::Arc;

use swe_edge_ingress_grpc::{CompressionMode, GrpcIngressInterceptorChain, NoopAuditSink};
use edge_domain_security::IngressTlsConfig;
use swe_edge_runtime_grpc::{NoopGrpcIngress, TonicGrpcServerBuilder};

fn handler() -> Arc<NoopGrpcIngress> {
    NoopGrpcIngress::create()
}

fn builder() -> TonicGrpcServerBuilder {
    TonicGrpcServerBuilder::new("127.0.0.1:0", handler())
}

// ── new ─────────────────────────────────────────────────────────────────────

#[test]
fn test_new_sets_bind_field_happy() {
    // @covers: new
    let s = TonicGrpcServerBuilder::new("127.0.0.1:8080", handler()).build();
    assert_eq!(s.bind_addr(), "127.0.0.1:8080");
}

#[test]
fn test_new_default_reflection_off_error() {
    // @covers: new
    let s = builder().build();
    assert!(
        !s.is_reflection_enabled(),
        "new builder must have reflection disabled"
    );
}

// ── with_max_message_size ───────────────────────────────────────────────────

#[test]
fn test_with_max_message_size_stores_value_happy() {
    // @covers: with_max_message_size
    let s = builder().with_max_message_size(512).build();
    assert_eq!(s.max_message_size(), 512);
}

#[test]
fn test_with_max_message_size_default_is_none_error() {
    // @covers: with_max_message_size
    let s = builder().build();
    assert_ne!(
        s.max_message_size(),
        0,
        "default built server must have nonzero max message size"
    );
}

#[test]
fn test_with_max_message_size_value_of_one_edge() {
    // @covers: with_max_message_size
    let s = builder().with_max_message_size(1).build();
    assert_eq!(s.max_message_size(), 1, "must accept 1 byte as minimum");
    assert_ne!(s.max_message_size(), 0);
}

// ── with_max_concurrent_streams ─────────────────────────────────────────────

#[test]
fn test_with_max_concurrent_streams_stores_value_happy() {
    // @covers: with_max_concurrent_streams
    let s = builder().with_max_concurrent_streams(4).build();
    assert_eq!(s.max_concurrent_streams(), 4);
}

#[test]
fn test_with_max_concurrent_streams_default_is_none_error() {
    // @covers: with_max_concurrent_streams
    let s = builder().build();
    assert_ne!(
        s.max_concurrent_streams(),
        0,
        "default built server must have nonzero stream limit"
    );
}

#[test]
fn test_with_max_concurrent_streams_value_of_one_edge() {
    // @covers: with_max_concurrent_streams
    let s = builder().with_max_concurrent_streams(1).build();
    assert_eq!(s.max_concurrent_streams(), 1, "must accept 1 as minimum");
    assert_ne!(s.max_concurrent_streams(), 0);
}

// ── with_tls ────────────────────────────────────────────────────────────────

#[test]
fn test_with_tls_stores_config_happy() {
    // @covers: with_tls
    let tls = IngressTlsConfig { cert_pem_path: "c.pem".into(), key_pem_path: "k.pem".into(), client_ca_pem_path: None };
    let s = builder().with_tls(tls).build();
    assert!(
        s.tls_config().is_some(),
        "with_tls must store the TLS config"
    );
    assert!(
        !s.is_unauthenticated_allowed(),
        "with_tls must not change allow_unauthenticated"
    );
}

#[test]
fn test_with_tls_default_is_none_error() {
    // @covers: with_tls
    let s = builder().build();
    assert!(
        s.tls_config().is_none(),
        "new builder must produce server with no TLS"
    );
}

#[test]
fn test_with_tls_overwrites_previous_edge() {
    // @covers: with_tls
    let tls1 = IngressTlsConfig { cert_pem_path: "a.pem".into(), key_pem_path: "b.pem".into(), client_ca_pem_path: None };
    let tls2 = IngressTlsConfig { cert_pem_path: "c.pem".into(), key_pem_path: "d.pem".into(), client_ca_pem_path: None };
    let s = builder().with_tls(tls1).with_tls(tls2).build();
    assert!(
        s.tls_config().is_some(),
        "second with_tls must overwrite the first"
    );
    assert!(
        !s.is_unauthenticated_allowed(),
        "with_tls overwrite must not change allow_unauthenticated"
    );
}

// ── with_interceptors ───────────────────────────────────────────────────────

#[test]
fn test_with_interceptors_stores_chain_happy() {
    // @covers: with_interceptors
    let s = builder()
        .with_interceptors(GrpcIngressInterceptorChain::new())
        .build();
    assert!(
        !s.interceptor_chain().contains_authorization(),
        "empty chain must not have authz"
    );
    assert!(
        !s.is_unauthenticated_allowed(),
        "with_interceptors must not change allow_unauthenticated"
    );
}

#[test]
fn test_with_interceptors_default_is_none_error() {
    // @covers: with_interceptors
    let s = builder().build();
    assert!(
        !s.interceptor_chain().contains_authorization(),
        "default server must have no authz interceptor"
    );
}

#[test]
fn test_with_interceptors_overwrites_previous_edge() {
    // @covers: with_interceptors
    let s = builder()
        .with_interceptors(GrpcIngressInterceptorChain::new())
        .with_interceptors(GrpcIngressInterceptorChain::new())
        .build();
    assert!(
        !s.interceptor_chain().contains_authorization(),
        "second with_interceptors must still produce valid chain"
    );
    assert!(
        !s.is_unauthenticated_allowed(),
        "overwrite must not change allow_unauthenticated"
    );
}

// ── with_compression ────────────────────────────────────────────────────────

#[test]
fn test_with_compression_stores_mode_happy() {
    // @covers: with_compression
    let s = builder().with_compression(CompressionMode::Gzip).build();
    assert!(matches!(s.compression_mode(), CompressionMode::Gzip));
}

#[test]
fn test_with_compression_default_is_none_error() {
    // @covers: with_compression
    let s = builder().build();
    assert!(
        matches!(s.compression_mode(), CompressionMode::None),
        "default built server must have no compression"
    );
}

#[test]
fn test_with_compression_override_gzip_to_none_edge() {
    // @covers: with_compression
    let s = builder()
        .with_compression(CompressionMode::Gzip)
        .with_compression(CompressionMode::None)
        .build();
    assert!(
        matches!(s.compression_mode(), CompressionMode::None),
        "compression must be overridable to None"
    );
}

// ── allow_unauthenticated ───────────────────────────────────────────────────

#[test]
fn test_allow_unauthenticated_sets_flag_happy() {
    // @covers: allow_unauthenticated
    let s = builder().allow_unauthenticated().build();
    assert!(
        s.is_unauthenticated_allowed(),
        "allow_unauthenticated must set the flag"
    );
}

#[test]
fn test_allow_unauthenticated_default_is_false_error() {
    // @covers: allow_unauthenticated
    let s = builder().build();
    assert!(
        !s.is_unauthenticated_allowed(),
        "default builder must not allow unauthenticated"
    );
}

#[test]
fn test_allow_unauthenticated_combined_with_tls_edge() {
    // @covers: allow_unauthenticated
    let tls = IngressTlsConfig { cert_pem_path: "c.pem".into(), key_pem_path: "k.pem".into(), client_ca_pem_path: None };
    let s = builder().with_tls(tls).allow_unauthenticated().build();
    assert!(
        s.is_unauthenticated_allowed(),
        "allow_unauthenticated must work alongside TLS"
    );
    assert!(s.tls_config().is_some(), "TLS must survive chaining");
}

// ── with_audit_sink ─────────────────────────────────────────────────────────

#[test]
fn test_with_audit_sink_stores_sink_happy() {
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
    let s = builder()
        .with_audit_sink(Arc::new(Counter(count.clone())))
        .build();
    s.audit_sink_ref().record(AuditEvent {
        timestamp: std::time::SystemTime::UNIX_EPOCH,
        method: "/test".into(),
        identity: None,
        status: GrpcStatusCode::Ok,
        duration_ms: 0,
    });
    assert_eq!(
        *count.lock().unwrap(),
        1,
        "with_audit_sink must store the sink for use by built server"
    );
    assert!(
        !s.is_unauthenticated_allowed(),
        "with_audit_sink must not change allow_unauthenticated"
    );
}

#[test]
fn test_with_audit_sink_default_is_none_error() {
    // @covers: with_audit_sink
    // Default builder produces a server with a no-op audit sink that does not panic on record
    use swe_edge_ingress_grpc::{AuditEvent, GrpcStatusCode};
    let s = builder().build();
    s.audit_sink_ref().record(AuditEvent {
        timestamp: std::time::SystemTime::UNIX_EPOCH,
        method: "/test".into(),
        identity: None,
        status: GrpcStatusCode::Ok,
        duration_ms: 0,
    });
    assert!(
        !s.is_unauthenticated_allowed(),
        "default builder must not allow unauthenticated"
    );
}

#[test]
fn test_with_audit_sink_overwrites_previous_edge() {
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
    let s = builder()
        .with_audit_sink(Arc::new(NoopAuditSink))
        .with_audit_sink(Arc::new(Counter(count.clone())))
        .build();
    s.audit_sink_ref().record(AuditEvent {
        timestamp: std::time::SystemTime::UNIX_EPOCH,
        method: "/test".into(),
        identity: None,
        status: GrpcStatusCode::Ok,
        duration_ms: 0,
    });
    assert_eq!(
        *count.lock().unwrap(),
        1,
        "second with_audit_sink must overwrite the first"
    );
    assert!(
        !s.is_unauthenticated_allowed(),
        "overwrite must not change allow_unauthenticated"
    );
}

// ── enable_reflection ───────────────────────────────────────────────────────

#[test]
fn test_enable_reflection_sets_flag_happy() {
    // @covers: enable_reflection
    let s = builder().enable_reflection().build();
    assert!(
        s.is_reflection_enabled(),
        "enable_reflection must set the flag"
    );
}

#[test]
fn test_enable_reflection_default_is_false_error() {
    // @covers: enable_reflection
    let s = builder().build();
    assert!(
        !s.is_reflection_enabled(),
        "new builder must not have reflection enabled"
    );
}

#[test]
fn test_enable_reflection_combined_with_tls_edge() {
    // @covers: enable_reflection
    let tls = IngressTlsConfig { cert_pem_path: "c.pem".into(), key_pem_path: "k.pem".into(), client_ca_pem_path: None };
    let s = builder().with_tls(tls).enable_reflection().build();
    assert!(
        s.is_reflection_enabled(),
        "enable_reflection must work alongside TLS"
    );
    assert!(s.tls_config().is_some(), "TLS must survive chaining");
}

// ── build ───────────────────────────────────────────────────────────────────

#[test]
fn test_build_propagates_reflection_flag_happy() {
    // @covers: build
    let s = builder()
        .allow_unauthenticated()
        .enable_reflection()
        .build();
    assert!(
        s.is_reflection_enabled(),
        "build must propagate reflection flag to server"
    );
}

#[test]
fn test_build_default_no_reflection_error() {
    // @covers: build
    let s = builder().allow_unauthenticated().build();
    assert!(
        !s.is_reflection_enabled(),
        "build without enable_reflection must have reflection off"
    );
}

#[test]
fn test_build_with_max_message_size_propagated_edge() {
    // @covers: build
    let s = builder()
        .allow_unauthenticated()
        .with_max_message_size(1)
        .build();
    assert_eq!(
        s.max_message_size(),
        1,
        "build must propagate minimum message size"
    );
    assert_ne!(s.max_message_size(), 0);
}
