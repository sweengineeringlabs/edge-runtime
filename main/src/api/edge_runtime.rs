//! `EdgeRuntime` builder — single entry-point for assembling and running an edge server.

use std::sync::Arc;

use edge_proxy::LifecycleMonitor;
use swe_edge_egress_grpc::GrpcOutbound;
use swe_edge_egress_http::HttpOutbound;
use swe_edge_ingress::{GrpcInbound, HttpInbound};

use crate::api::service_registry::ServiceRegistry;
use crate::api::types::RuntimeConfig;

/// Builder for assembling and starting an edge runtime.
///
/// Call [`EdgeRuntime::builder`] to construct one, wire in handlers and egress
/// clients, then call [`EdgeRuntimeBuilder::serve`] to start the servers.
///
/// ```rust,ignore
/// EdgeRuntime::builder()
///     .config(cfg)
///     .http_handler(Arc::new(my_http_handler))
///     .egress_http(Arc::new(http_client))
///     .serve()
///     .await?;
/// ```
pub struct EdgeRuntimeBuilder {
    pub(crate) config:        Option<RuntimeConfig>,
    pub(crate) http_handler:  Option<Arc<dyn HttpInbound>>,
    pub(crate) grpc_handler:  Option<Arc<dyn GrpcInbound>>,
    pub(crate) egress_http:   Option<Arc<dyn HttpOutbound>>,
    pub(crate) egress_grpc:   Option<Arc<dyn GrpcOutbound>>,
    pub(crate) lifecycle:     Option<Arc<dyn LifecycleMonitor>>,
}

/// Entry-point for the edge runtime.
///
/// Use [`EdgeRuntime::builder`] to obtain a [`EdgeRuntimeBuilder`].
pub struct EdgeRuntime;

impl EdgeRuntime {
    /// Start building an edge runtime.
    pub fn builder() -> EdgeRuntimeBuilder {
        EdgeRuntimeBuilder {
            config:       None,
            http_handler: None,
            grpc_handler: None,
            egress_http:  None,
            egress_grpc:  None,
            lifecycle:    None,
        }
    }
}

impl EdgeRuntimeBuilder {
    /// Set the runtime configuration.
    pub fn config(mut self, config: RuntimeConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// Register an HTTP inbound handler.
    pub fn http_handler(mut self, handler: Arc<dyn HttpInbound>) -> Self {
        self.http_handler = Some(handler);
        self
    }

    /// Register a gRPC inbound handler.
    pub fn grpc_handler(mut self, handler: Arc<dyn GrpcInbound>) -> Self {
        self.grpc_handler = Some(handler);
        self
    }

    /// Register the HTTP egress client (used for outbound HTTP calls).
    pub fn egress_http(mut self, client: Arc<dyn HttpOutbound>) -> Self {
        self.egress_http = Some(client);
        self
    }

    /// Register the gRPC egress client (used for outbound gRPC calls).
    pub fn egress_grpc(mut self, client: Arc<dyn GrpcOutbound>) -> Self {
        self.egress_grpc = Some(client);
        self
    }

    /// Set the lifecycle monitor.  Defaults to a null (no-op) monitor when omitted.
    pub fn lifecycle(mut self, monitor: Arc<dyn LifecycleMonitor>) -> Self {
        self.lifecycle = Some(monitor);
        self
    }

    /// Build the `ServiceRegistry` from currently registered egress clients.
    ///
    /// Call this before `.serve()` if you need to hand the registry to handlers
    /// at construction time.
    pub fn build_registry(&self) -> Option<Arc<ServiceRegistry>> {
        self.egress_http.as_ref().map(|http| {
            Arc::new(ServiceRegistry::new(
                Arc::clone(http),
                self.egress_grpc.clone(),
            ))
        })
    }
}
