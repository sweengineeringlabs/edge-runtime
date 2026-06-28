//! GrpcServerBuild trait impl for TonicGrpcServerBuilder.
use std::sync::Arc;

use edge_domain_security::IngressTlsConfig;
use swe_edge_ingress_grpc::{AuditSink, CompressionMode, GrpcIngress, GrpcIngressInterceptorChain};

use crate::api::{
    GrpcServerBuild, GrpcServerManage, TonicGrpcServer, TonicGrpcServerBuilder,
};

impl GrpcServerBuild for TonicGrpcServerBuilder {
    fn new(bind: impl Into<String>, handler: Arc<dyn GrpcIngress>) -> Self {
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

    fn with_max_message_size(mut self, size: usize) -> Self {
        self.max_bytes = Some(size);
        self
    }

    fn with_max_concurrent_streams(mut self, streams: u32) -> Self {
        self.max_concurrent_streams = Some(streams);
        self
    }

    fn with_tls(mut self, cfg: IngressTlsConfig) -> Self {
        self.tls = Some(cfg);
        self
    }

    fn with_interceptors(mut self, chain: GrpcIngressInterceptorChain) -> Self {
        self.interceptors = Some(chain);
        self
    }

    fn with_compression(mut self, mode: CompressionMode) -> Self {
        self.compression = Some(mode);
        self
    }

    fn allow_unauthenticated(mut self) -> Self {
        self.allow_unauthenticated = true;
        self
    }

    fn with_audit_sink(mut self, sink: Arc<dyn AuditSink>) -> Self {
        self.audit_sink = Some(sink);
        self
    }

    fn enable_reflection(mut self) -> Self {
        self.enable_reflection = true;
        self
    }

    fn build(self) -> TonicGrpcServer {
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

    fn builder_bind(&self) -> &str {
        &self.bind
    }
}
