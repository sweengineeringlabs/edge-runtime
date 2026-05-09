//! `EdgeRuntime` builder — single entry-point for assembling and running an edge server.

use std::sync::Arc;

use edge_domain::{Handler, HandlerError, HandlerRegistry};
use edge_proxy::LifecycleMonitor;
use swe_edge_egress_grpc::GrpcOutbound;
use swe_edge_egress_http::HttpOutbound;
use swe_edge_ingress::{
    GrpcDecodeFn, GrpcEncodeFn, GrpcHandlerAdapter, GrpcHandlerRegistryDispatcher,
    GrpcInbound, GrpcInboundInterceptor, GrpcInboundInterceptorChain,
    GrpcRequest, GrpcResponse,
    HttpDecodeFn, HttpEncodeFn, HttpHandlerAdapter, HttpHandlerRegistryDispatcher,
    HttpInbound, HttpRequest, HttpResponse, IngressTlsConfig,
};
use swe_edge_ingress_verifier::TokenVerifier;

use crate::api::service_registry::ServiceRegistry;
use crate::api::types::RuntimeConfig;

/// Builder for assembling and starting an edge runtime.
///
/// Config loading when `.config()` is not called:
/// XDG Base Directory chain is used with `app_name` (default `"swe-edge"`):
/// `/etc/xdg/<app_name>/application.toml` → `~/.config/<app_name>/application.toml`
/// → `$SWE_EDGE_CONFIG_DIR/application.toml` → env vars.
///
/// ```rust,ignore
/// EdgeRuntime::builder()
///     .app_name("my-service")          // sets XDG app dir; skip to use "swe-edge"
///     .http_tls(IngressTlsConfig::tls("cert.pem", "key.pem"))
///     .http_bearer_auth(Arc::new(JwtVerifier::from_config(&jwt_cfg)?))
///     .http_route(Arc::new(MyHandler), decode, encode)
///     .egress_http(Arc::new(http_client))
///     .serve()
///     .await?;
/// ```
pub struct EdgeRuntimeBuilder {
    pub(crate) config:                   Option<RuntimeConfig>,
    pub(crate) app_name:                 Option<String>,
    // Inbound handlers (explicit escape hatch)
    pub(crate) http_handler:             Option<Arc<dyn HttpInbound>>,
    pub(crate) grpc_handler:             Option<Arc<dyn GrpcInbound>>,
    // Accumulated route dispatchers (built by http_route / grpc_route)
    pub(crate) http_dispatcher:          Option<HttpHandlerRegistryDispatcher>,
    pub(crate) grpc_dispatcher:          Option<GrpcHandlerRegistryDispatcher>,
    // TLS / mTLS
    pub(crate) http_tls:                 Option<IngressTlsConfig>,
    pub(crate) grpc_tls:                 Option<IngressTlsConfig>,
    // HTTP JWT bearer auth
    pub(crate) http_bearer_verifier:     Option<Arc<dyn TokenVerifier>>,
    // gRPC auth interceptors
    pub(crate) grpc_interceptors:        GrpcInboundInterceptorChain,
    pub(crate) grpc_allow_unauthenticated: bool,
    // Egress
    pub(crate) egress_http:              Option<Arc<dyn HttpOutbound>>,
    pub(crate) egress_grpc:              Option<Arc<dyn GrpcOutbound>>,
    // Lifecycle
    pub(crate) lifecycle:                Option<Arc<dyn LifecycleMonitor>>,
}

/// Entry-point for the edge runtime.
pub struct EdgeRuntime;

impl EdgeRuntime {
    pub fn builder() -> EdgeRuntimeBuilder {
        EdgeRuntimeBuilder {
            config:                    None,
            app_name:                  None,
            http_handler:              None,
            grpc_handler:              None,
            http_dispatcher:           None,
            grpc_dispatcher:           None,
            http_tls:                  None,
            grpc_tls:                  None,
            http_bearer_verifier:      None,
            grpc_interceptors:         GrpcInboundInterceptorChain::new(),
            grpc_allow_unauthenticated: false,
            egress_http:               None,
            egress_grpc:               None,
            lifecycle:                 None,
        }
    }
}

impl EdgeRuntimeBuilder {
    /// Set the runtime configuration directly, bypassing XDG file loading.
    pub fn config(mut self, config: RuntimeConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// Override the XDG application name used for config file discovery.
    ///
    /// When `.config()` is not called, `serve()` loads from the XDG chain
    /// at `$XDG_CONFIG_HOME/<app_name>/application.toml` (and the system
    /// equivalents).  Defaults to `"swe-edge"`.
    pub fn app_name(mut self, name: impl Into<String>) -> Self {
        self.app_name = Some(name.into());
        self
    }

    // ── Routing ───────────────────────────────────────────────────────────────

    /// Register an HTTP handler.  Routes accumulate in an internal dispatcher.
    pub fn http_route<Req, Resp>(
        mut self,
        handler: Arc<dyn Handler<Req, Resp>>,
        decode:  HttpDecodeFn<Req>,
        encode:  HttpEncodeFn<Resp>,
    ) -> Self
    where
        Req:  Send + 'static,
        Resp: Send + 'static,
    {
        let d = self.http_dispatcher.get_or_insert_with(|| {
            HttpHandlerRegistryDispatcher::new(Arc::new(HandlerRegistry::new()))
        });
        d.register(HttpHandlerAdapter::new(handler, decode, encode))
            .expect("duplicate HTTP route");
        self
    }

    /// Register a gRPC handler.  Routes accumulate in an internal dispatcher.
    pub fn grpc_route<Req, Resp>(
        mut self,
        handler: Arc<dyn Handler<Req, Resp>>,
        decode:  GrpcDecodeFn<Req>,
        encode:  GrpcEncodeFn<Resp>,
    ) -> Self
    where
        Req:  Send + 'static,
        Resp: Send + 'static,
    {
        let d = self.grpc_dispatcher.get_or_insert_with(|| {
            GrpcHandlerRegistryDispatcher::new(Arc::new(HandlerRegistry::new()))
        });
        d.register(GrpcHandlerAdapter::new(handler, decode, encode));
        self
    }

    // ── TLS / mTLS ────────────────────────────────────────────────────────────

    /// Enable TLS or mTLS for the HTTP server.
    ///
    /// - One-way TLS:  `IngressTlsConfig::tls("cert.pem", "key.pem")`
    /// - mTLS:         `IngressTlsConfig::mtls("cert.pem", "key.pem", "ca.pem")`
    pub fn http_tls(mut self, config: IngressTlsConfig) -> Self {
        self.http_tls = Some(config);
        self
    }

    /// Enable TLS or mTLS for the gRPC server.
    pub fn grpc_tls(mut self, config: IngressTlsConfig) -> Self {
        self.grpc_tls = Some(config);
        self
    }

    // ── Authentication ────────────────────────────────────────────────────────

    /// Require JWT bearer tokens on every HTTP request.
    ///
    /// Requests without a valid `Authorization: Bearer <token>` header receive
    /// `401 Unauthorized`.  The verified identity is available in
    /// `ctx.subject` / `ctx.tenant_id` inside the handler.
    ///
    /// Use `JwtVerifier::from_config(&jwt_cfg)` to build the verifier from
    /// configuration.
    pub fn http_bearer_auth(mut self, verifier: Arc<dyn TokenVerifier>) -> Self {
        self.http_bearer_verifier = Some(verifier);
        self
    }

    /// Register a gRPC auth interceptor (e.g. a bearer or mTLS interceptor).
    ///
    /// Multiple interceptors run in registration order; the first failure
    /// short-circuits and the handler is never invoked.
    pub fn grpc_auth(mut self, interceptor: Arc<dyn GrpcInboundInterceptor>) -> Self {
        self.grpc_interceptors = self.grpc_interceptors.push(interceptor);
        self
    }

    /// Allow gRPC requests without any registered auth interceptor.
    ///
    /// By default the gRPC server enforces fail-closed auth — at least one
    /// interceptor marked as an `AuthorizationInterceptor` must be registered.
    /// Call this to opt out (development / internal services only).
    pub fn grpc_allow_unauthenticated(mut self) -> Self {
        self.grpc_allow_unauthenticated = true;
        self
    }

    // ── Egress ────────────────────────────────────────────────────────────────

    /// Register the HTTP egress client.
    pub fn egress_http(mut self, client: Arc<dyn HttpOutbound>) -> Self {
        self.egress_http = Some(client);
        self
    }

    /// Register the gRPC egress client.
    pub fn egress_grpc(mut self, client: Arc<dyn GrpcOutbound>) -> Self {
        self.egress_grpc = Some(client);
        self
    }

    // ── Lifecycle ─────────────────────────────────────────────────────────────

    /// Set the lifecycle monitor.  Defaults to a null (no-op) monitor.
    pub fn lifecycle(mut self, monitor: Arc<dyn LifecycleMonitor>) -> Self {
        self.lifecycle = Some(monitor);
        self
    }

    // ── Escape hatches ────────────────────────────────────────────────────────

    /// Supply a pre-built HTTP inbound handler (prefer `http_route`).
    pub fn http_handler(mut self, handler: Arc<dyn HttpInbound>) -> Self {
        self.http_handler = Some(handler);
        self
    }

    /// Supply a pre-built gRPC inbound handler (prefer `grpc_route`).
    pub fn grpc_handler(mut self, handler: Arc<dyn GrpcInbound>) -> Self {
        self.grpc_handler = Some(handler);
        self
    }

    /// Build the egress `ServiceRegistry` to pass to handler constructors.
    pub fn build_registry(&self) -> Option<Arc<ServiceRegistry>> {
        self.egress_http.as_ref().map(|http| {
            Arc::new(ServiceRegistry::new(Arc::clone(http), self.egress_grpc.clone()))
        })
    }
}
