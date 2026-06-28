//! TonicGrpcServerBuilder type declaration.

use std::sync::Arc;

use edge_domain_security::IngressTlsConfig;
use swe_edge_ingress_grpc::{AuditSink, CompressionMode, GrpcIngress, GrpcIngressInterceptorChain};

/// Fluent builder for [`TonicGrpcServer`].
pub struct TonicGrpcServerBuilder {
    pub(crate) bind: String,
    pub(crate) handler: Arc<dyn GrpcIngress>,
    pub(crate) max_bytes: Option<usize>,
    pub(crate) max_concurrent_streams: Option<u32>,
    pub(crate) tls: Option<IngressTlsConfig>,
    pub(crate) interceptors: Option<GrpcIngressInterceptorChain>,
    pub(crate) compression: Option<CompressionMode>,
    pub(crate) allow_unauthenticated: bool,
    pub(crate) audit_sink: Option<Arc<dyn AuditSink>>,
    pub(crate) enable_reflection: bool,
}
