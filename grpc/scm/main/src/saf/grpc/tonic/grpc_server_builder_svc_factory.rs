//! SAF factory surface — TonicGrpcServerBuilder inherent methods.

use std::sync::Arc;

use swe_edge_ingress_grpc::{AuditSink, CompressionMode, GrpcIngress, GrpcIngressInterceptorChain};
use edge_domain_security::IngressTlsConfig;

use crate::api::{TonicGrpcServer, TonicGrpcServerBuilder};

impl TonicGrpcServerBuilder {
    /// Start building a server bound to `bind` that delegates to `handler`.
    pub fn new(bind: impl Into<String>, handler: Arc<dyn GrpcIngress>) -> Self {
        Self {
            bind: bind.into(),
            handler,
            max_bytes: None,
            max_concurrent_streams: None,
            tls: None,
            interceptors: None,
            compression: None,
            allow_unauthenticated: false,
            audit_sink: None,
            enable_reflection: false,
        }
    }

    /// Override the maximum inbound message size in bytes.
    pub fn with_max_message_size(mut self, size: usize) -> Self {
        self.max_bytes = Some(size);
        self
    }

    /// Override the maximum number of concurrent HTTP/2 streams.
    pub fn with_max_concurrent_streams(mut self, streams: u32) -> Self {
        self.max_concurrent_streams = Some(streams);
        self
    }

    /// Attach a TLS configuration.
    pub fn with_tls(mut self, cfg: IngressTlsConfig) -> Self {
        self.tls = Some(cfg);
        self
    }

    /// Attach an interceptor chain.
    pub fn with_interceptors(mut self, chain: GrpcIngressInterceptorChain) -> Self {
        self.interceptors = Some(chain);
        self
    }

    /// Set the compression mode.
    pub fn with_compression(mut self, mode: CompressionMode) -> Self {
        self.compression = Some(mode);
        self
    }

    /// Allow unauthenticated callers.
    pub fn allow_unauthenticated(mut self) -> Self {
        self.allow_unauthenticated = true;
        self
    }

    /// Replace the default no-op audit sink.
    pub fn with_audit_sink(mut self, sink: Arc<dyn AuditSink>) -> Self {
        self.audit_sink = Some(sink);
        self
    }

    /// Enable gRPC reflection.
    pub fn enable_reflection(mut self) -> Self {
        self.enable_reflection = true;
        self
    }

    /// Consume the builder and produce a [`TonicGrpcServer`].
    pub fn build(self) -> TonicGrpcServer {
        let mut s = TonicGrpcServer::new(self.bind, self.handler);
        if let Some(v) = self.max_bytes {
            s = s.with_max_message_size(v);
        }
        if let Some(v) = self.max_concurrent_streams {
            s = s.with_max_concurrent_streams(v);
        }
        if let Some(v) = self.tls {
            s = s.with_tls(v);
        }
        if let Some(v) = self.interceptors {
            s = s.with_interceptors(v);
        }
        if let Some(v) = self.compression {
            s = s.with_compression(v);
        }
        s = s.allow_unauthenticated(self.allow_unauthenticated);
        if let Some(v) = self.audit_sink {
            s = s.with_audit_sink(v);
        }
        s = s.enable_reflection(self.enable_reflection);
        s
    }
}
