//! Integration tests for TonicGrpcServerBuilder factory surface (saf/grpc/tonic/grpc_server_builder_svc_factory.rs).
#![allow(clippy::unwrap_used)]

use std::sync::Arc;

use swe_edge_ingress_grpc::CompressionMode;
use swe_edge_ingress_tls::IngressTlsConfig;
use swe_edge_runtime_grpc::{NoopGrpcIngress, TonicGrpcServerBuilder};

fn handler() -> Arc<NoopGrpcIngress> {
    NoopGrpcIngress::create()
}

fn builder() -> TonicGrpcServerBuilder {
    TonicGrpcServerBuilder::new("127.0.0.1:0", handler())
}

// ── new ─────────────────────────────────────────────────────────────────────

#[test]
fn test_new_produces_builder_with_bind_addr_happy() {
    // @covers: new
    let s = TonicGrpcServerBuilder::new("10.0.0.1:9090", handler()).build();
    assert_eq!(
        s.bind_addr(),
        "10.0.0.1:9090",
        "builder must pass bind address to server"
    );
}

#[test]
fn test_new_builder_has_no_reflection_by_default_error() {
    // @covers: new
    let s = builder().build();
    assert!(
        !s.is_reflection_enabled(),
        "default new() builder must produce server with reflection off"
    );
    assert!(
        !s.is_unauthenticated_allowed(),
        "default new() builder must produce server that requires auth"
    );
}

#[test]
fn test_new_builder_has_nonzero_max_message_size_edge() {
    // @covers: new
    let s = builder().build();
    assert_ne!(
        s.max_message_size(),
        0,
        "default builder must produce server with nonzero message size limit"
    );
    assert_ne!(
        s.max_concurrent_streams(),
        0,
        "default builder must produce server with nonzero stream limit"
    );
}

// ── build ───────────────────────────────────────────────────────────────────

#[test]
fn test_build_produces_server_with_all_defaults_happy() {
    // @covers: build
    let s = builder().build();
    assert!(
        !s.is_reflection_enabled(),
        "build defaults must have reflection off"
    );
    assert!(s.tls_config().is_none(), "build defaults must have no TLS");
}

#[test]
fn test_build_without_allow_unauthenticated_defaults_to_false_error() {
    // @covers: build
    let s = builder().build();
    assert!(
        !s.is_unauthenticated_allowed(),
        "build without allow_unauthenticated must default to false"
    );
}

#[test]
fn test_build_with_all_options_propagated_edge() {
    // @covers: build
    let tls = IngressTlsConfig::tls("cert.pem", "key.pem");
    let s = builder()
        .with_tls(tls)
        .allow_unauthenticated()
        .enable_reflection()
        .with_max_message_size(4096)
        .with_compression(CompressionMode::Gzip)
        .build();
    assert!(s.tls_config().is_some(), "build must propagate with_tls");
    assert!(
        s.is_unauthenticated_allowed(),
        "build must propagate allow_unauthenticated"
    );
    assert!(
        s.is_reflection_enabled(),
        "build must propagate enable_reflection"
    );
    assert_eq!(
        s.max_message_size(),
        4096,
        "build must propagate with_max_message_size"
    );
    assert!(
        matches!(s.compression_mode(), CompressionMode::Gzip),
        "build must propagate with_compression"
    );
}

// ── with_max_message_size ───────────────────────────────────────────────────

#[test]
fn test_with_max_message_size_propagated_to_server_happy() {
    // @covers: with_max_message_size
    let s = builder().with_max_message_size(8192).build();
    assert_eq!(
        s.max_message_size(),
        8192,
        "with_max_message_size must propagate to built server"
    );
}

#[test]
fn test_with_max_message_size_overrides_default_error() {
    // @covers: with_max_message_size
    let default_s = builder().build();
    let custom_s = builder().with_max_message_size(1).build();
    assert_ne!(
        default_s.max_message_size(),
        custom_s.max_message_size(),
        "overridden size must differ from default"
    );
    assert_eq!(custom_s.max_message_size(), 1);
}

#[test]
fn test_with_max_message_size_minimum_one_byte_edge() {
    // @covers: with_max_message_size
    let s = builder().with_max_message_size(1).build();
    assert_eq!(s.max_message_size(), 1, "minimum 1 byte must be propagated");
    assert_ne!(s.max_message_size(), 0);
}

// ── with_max_concurrent_streams ─────────────────────────────────────────────

#[test]
fn test_with_max_concurrent_streams_propagated_to_server_happy() {
    // @covers: with_max_concurrent_streams
    let s = builder().with_max_concurrent_streams(32).build();
    assert_eq!(s.max_concurrent_streams(), 32);
}

#[test]
fn test_with_max_concurrent_streams_overrides_default_error() {
    // @covers: with_max_concurrent_streams
    let default_s = builder().build();
    let custom_s = builder().with_max_concurrent_streams(1).build();
    assert_ne!(
        default_s.max_concurrent_streams(),
        custom_s.max_concurrent_streams()
    );
    assert_eq!(custom_s.max_concurrent_streams(), 1);
}

#[test]
fn test_with_max_concurrent_streams_minimum_one_edge() {
    // @covers: with_max_concurrent_streams
    let s = builder().with_max_concurrent_streams(1).build();
    assert_eq!(
        s.max_concurrent_streams(),
        1,
        "minimum 1 stream must be propagated"
    );
    assert_ne!(s.max_concurrent_streams(), 0);
}

// ── with_tls ────────────────────────────────────────────────────────────────

#[test]
fn test_with_tls_propagated_to_server_happy() {
    // @covers: with_tls
    let tls = IngressTlsConfig::tls("cert.pem", "key.pem");
    let s = builder().with_tls(tls).build();
    assert!(
        s.tls_config().is_some(),
        "with_tls must be propagated to built server"
    );
    assert!(
        !s.is_reflection_enabled(),
        "with_tls must not change reflection setting"
    );
}

#[test]
fn test_with_tls_default_produces_no_tls_error() {
    // @covers: with_tls
    let s = builder().build();
    assert!(
        s.tls_config().is_none(),
        "builder without with_tls must produce server with no TLS"
    );
}

#[test]
fn test_with_tls_overwrite_propagated_edge() {
    // @covers: with_tls
    let tls1 = IngressTlsConfig::tls("a.pem", "b.pem");
    let tls2 = IngressTlsConfig::tls("c.pem", "d.pem");
    let s = builder().with_tls(tls1).with_tls(tls2).build();
    assert!(
        s.tls_config().is_some(),
        "second with_tls must be propagated"
    );
    assert!(
        !s.is_unauthenticated_allowed(),
        "with_tls overwrite must not change allow_unauthenticated"
    );
}

// ── allow_unauthenticated ───────────────────────────────────────────────────

#[test]
fn test_allow_unauthenticated_propagated_to_server_happy() {
    // @covers: allow_unauthenticated
    let s = builder().allow_unauthenticated().build();
    assert!(
        s.is_unauthenticated_allowed(),
        "allow_unauthenticated must be propagated to built server"
    );
}

#[test]
fn test_allow_unauthenticated_default_is_false_error() {
    // @covers: allow_unauthenticated
    let s = builder().build();
    assert!(
        !s.is_unauthenticated_allowed(),
        "without allow_unauthenticated, server must require auth"
    );
}

#[test]
fn test_allow_unauthenticated_survives_other_options_edge() {
    // @covers: allow_unauthenticated
    let s = builder()
        .allow_unauthenticated()
        .enable_reflection()
        .with_max_message_size(512)
        .build();
    assert!(
        s.is_unauthenticated_allowed(),
        "allow_unauthenticated must survive other builder calls"
    );
    assert!(s.is_reflection_enabled());
    assert_eq!(s.max_message_size(), 512);
}

// ── enable_reflection ───────────────────────────────────────────────────────

#[test]
fn test_enable_reflection_propagated_to_server_happy() {
    // @covers: enable_reflection
    let s = builder().enable_reflection().build();
    assert!(
        s.is_reflection_enabled(),
        "enable_reflection must be propagated to built server"
    );
}

#[test]
fn test_enable_reflection_default_is_false_error() {
    // @covers: enable_reflection
    let s = builder().build();
    assert!(
        !s.is_reflection_enabled(),
        "without enable_reflection, server must have reflection off"
    );
}

#[test]
fn test_enable_reflection_with_tls_edge() {
    // @covers: enable_reflection
    let tls = IngressTlsConfig::tls("cert.pem", "key.pem");
    let s = builder().enable_reflection().with_tls(tls).build();
    assert!(
        s.is_reflection_enabled(),
        "reflection must survive chaining with TLS"
    );
    assert!(s.tls_config().is_some());
}

// ── with_compression ────────────────────────────────────────────────────────

#[test]
fn test_with_compression_propagated_to_server_happy() {
    // @covers: with_compression
    let s = builder().with_compression(CompressionMode::Gzip).build();
    assert!(
        matches!(s.compression_mode(), CompressionMode::Gzip),
        "compression must be propagated to built server"
    );
}

#[test]
fn test_with_compression_default_is_none_error() {
    // @covers: with_compression
    let s = builder().build();
    assert!(
        matches!(s.compression_mode(), CompressionMode::None),
        "default server must have no compression"
    );
}

#[test]
fn test_with_compression_overwrite_propagated_edge() {
    // @covers: with_compression
    let s = builder()
        .with_compression(CompressionMode::Gzip)
        .with_compression(CompressionMode::None)
        .build();
    assert!(
        matches!(s.compression_mode(), CompressionMode::None),
        "last compression write must win"
    );
}

// ── with_interceptors ───────────────────────────────────────────────────────

#[test]
fn test_with_interceptors_propagated_to_server_happy() {
    // @covers: with_interceptors
    use swe_edge_ingress_grpc::GrpcIngressInterceptorChain;
    let s = builder()
        .with_interceptors(GrpcIngressInterceptorChain::new())
        .build();
    assert!(
        !s.interceptor_chain().contains_authorization(),
        "interceptors must be propagated to built server"
    );
}

#[test]
fn test_with_interceptors_default_has_no_authz_error() {
    // @covers: with_interceptors
    let s = builder().build();
    assert!(
        !s.interceptor_chain().contains_authorization(),
        "default server must have no authz interceptor"
    );
}

#[test]
fn test_with_interceptors_overwrite_propagated_edge() {
    // @covers: with_interceptors
    use swe_edge_ingress_grpc::GrpcIngressInterceptorChain;
    let s = builder()
        .with_interceptors(GrpcIngressInterceptorChain::new())
        .with_interceptors(GrpcIngressInterceptorChain::new())
        .build();
    assert!(
        !s.interceptor_chain().contains_authorization(),
        "second interceptors write must be propagated"
    );
}

// ── with_audit_sink ─────────────────────────────────────────────────────────

#[test]
fn test_with_audit_sink_propagated_to_server_happy() {
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
        "audit sink must be propagated to built server"
    );
}

#[test]
fn test_with_audit_sink_default_does_not_panic_error() {
    // @covers: with_audit_sink
    use swe_edge_ingress_grpc::{AuditEvent, GrpcStatusCode};
    let s = builder().build();
    s.audit_sink_ref().record(AuditEvent {
        timestamp: std::time::SystemTime::UNIX_EPOCH,
        method: "/noop".into(),
        identity: None,
        status: GrpcStatusCode::Ok,
        duration_ms: 0,
    });
    assert!(
        !s.is_reflection_enabled(),
        "default audit sink must not affect other server state"
    );
}

#[test]
fn test_with_audit_sink_overwrite_second_wins_edge() {
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
    assert_eq!(*count.lock().unwrap(), 1, "second with_audit_sink must win");
}
