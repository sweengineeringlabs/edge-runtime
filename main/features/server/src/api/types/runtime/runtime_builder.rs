//! `RuntimeBuilder` — fluent builder for assembling an edge runtime.

use std::sync::Arc;

use edge_proxy::LifecycleMonitor;
use swe_edge_egress_grpc::GrpcEgress;
use swe_edge_egress_http::HttpEgress;
use swe_edge_ingress_grpc::{
    GrpcHandlerRegistryDispatcher, GrpcIngress, GrpcIngressInterceptorChain,
};
use swe_edge_ingress_http::{
    HttpHandlerRegistryDispatcher, HttpIngress, HttpStream, IngressTlsConfig,
};
use swe_edge_ingress_verifier::TokenVerifier;

use crate::api::types::RuntimeConfig;

/// Builder for assembling and starting an edge runtime.
pub struct RuntimeBuilder {
    pub(crate) config: Option<RuntimeConfig>,
    pub(crate) app_name: Option<String>,
    pub(crate) http_handler: Option<Arc<dyn HttpIngress>>,
    pub(crate) grpc_handler: Option<Arc<dyn GrpcIngress>>,
    pub(crate) http_dispatcher: Option<HttpHandlerRegistryDispatcher>,
    pub(crate) grpc_dispatcher: Option<GrpcHandlerRegistryDispatcher>,
    pub(crate) http_tls: Option<IngressTlsConfig>,
    pub(crate) grpc_tls: Option<IngressTlsConfig>,
    pub(crate) http_bearer_verifier: Option<Arc<dyn TokenVerifier>>,
    pub(crate) grpc_interceptors: GrpcIngressInterceptorChain,
    pub(crate) grpc_allow_unauthenticated: bool,
    pub(crate) egress_http: Option<Arc<dyn HttpEgress>>,
    pub(crate) egress_grpc: Option<Arc<dyn GrpcEgress>>,
    pub(crate) lifecycle: Option<Arc<dyn LifecycleMonitor>>,
    pub(crate) tracing_config: Option<crate::api::config::TracingConfig>,
    pub(crate) stream_handler: Option<Arc<dyn HttpStream>>,
    #[cfg(feature = "message-broker")]
    pub(crate) message_broker: Option<Arc<dyn swe_edge_runtime_message_broker::MessageBroker>>,
}
