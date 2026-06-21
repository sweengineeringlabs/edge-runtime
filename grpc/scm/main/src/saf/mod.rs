//! SAF layer — gRPC server hosting public facade.

mod grpc_server_svc;

pub use crate::api::server::error::{GrpcServerConfigError, GrpcServerError};
pub use crate::api::server::traits::{GrpcServer, GrpcServerObserver};
pub use crate::api::server::types::{
    GrpcServerConfig, GrpcServerConfigBuilder, StatusCodeConverter, TonicGrpcServer,
    TonicGrpcServerBuilder, DEFAULT_KEEPALIVE_INTERVAL, DEFAULT_KEEPALIVE_INTERVAL_SECS,
    DEFAULT_KEEPALIVE_TIMEOUT, DEFAULT_KEEPALIVE_TIMEOUT_SECS, DEFAULT_MAX_CONCURRENT_STREAMS,
    DEFAULT_MAX_MESSAGE_BYTES, MAX_MESSAGE_BYTES, MISSING_AUTHORIZATION_INTERCEPTOR_MSG,
    REFLECTION_ENABLED_WARN_MSG, SANITIZED_INTERNAL_MSG,
};
pub use crate::api::server::PeerIdentityExtractor;
pub use crate::core::noop::{NoopGrpcIngress, NoopGrpcValidator, Validator};

// Re-export all ingress-grpc port contract types so consumers have a single import point.
pub use swe_edge_ingress_grpc::{
    AuditEvent, AuditEventBuilder, AuditSink, AuthorizationInterceptor, CompressionMode,
    GrpcDecodeFn, GrpcEncodeFn, GrpcHandlerAdapter, GrpcHandlerRegistryDispatcher, GrpcHealthCheck,
    GrpcIngress, GrpcIngressError, GrpcIngressInterceptor, GrpcIngressInterceptorChain,
    GrpcIngressResult, GrpcMessageStream, GrpcMetadata, GrpcRequest, GrpcRequestBuilder,
    GrpcResponse, GrpcStatusCode, GrpcTimeoutParser, HealthAggregate, HealthService,
    IngressLoadBalancer, LoadBalancerHint, NodeId, NoopAuditSink, PeerIdentity, SecurityContext,
    ServingStatus, TraceContextInterceptor, DEFAULT_DEADLINE, EXTRACTED_TRACEPARENT,
    EXTRACTED_TRACESTATE, HEALTH_CHECK_METHOD, HEALTH_WATCH_METHOD, PEER_CERT_FINGERPRINT_SHA256,
    PEER_CN, PEER_IDENTITY, PEER_SAN_DNS, PEER_SAN_URI, RESERVED_PEER_PREFIXES, TRACEPARENT,
    TRACESTATE, WATCH_CHANNEL_CAPACITY,
};
pub use swe_edge_ingress_tls::{IngressTlsConfig, IngressTlsError};

pub use grpc_server_svc::create_config_builder;
