//! SAF layer — gRPC server hosting public facade.

mod grpc;

pub use grpc::{
    GrpcServer, GrpcServerConfig, GrpcServerConfigBuilder, GrpcServerConfigError, GrpcServerError,
    GrpcServerObserverSvc, GrpcServerSvc, NoopGrpcIngress, NoopGrpcValidator, StatusCodeConverter,
    TonicGrpcServer, TonicGrpcServerBuilder, Validator, ValidatorSvc, DEFAULT_KEEPALIVE_INTERVAL,
    DEFAULT_KEEPALIVE_INTERVAL_SECS, DEFAULT_KEEPALIVE_TIMEOUT, DEFAULT_KEEPALIVE_TIMEOUT_SECS,
    DEFAULT_MAX_CONCURRENT_STREAMS, DEFAULT_MAX_MESSAGE_BYTES, MAX_MESSAGE_BYTES,
    MISSING_AUTHORIZATION_INTERCEPTOR_MSG, REFLECTION_ENABLED_WARN_MSG, SANITIZED_INTERNAL_MSG,
};

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
