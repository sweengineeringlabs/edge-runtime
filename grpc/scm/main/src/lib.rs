//! `swe-edge-runtime-grpc` — gRPC server hosting crate for the swe-edge runtime layer.
//!
//! Provides [`TonicGrpcServer`], [`GrpcServerConfig`], the full dispatch loop,
//! and re-exports all ingress-grpc port contract types.

#![warn(missing_docs)]
#![deny(unsafe_code)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used))]
mod api;
mod core;
mod saf;
mod spi;

pub use api::*;
pub use edge_domain_security::{IngressTlsConfig, IngressTlsError};
pub use saf::*;
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
