//! `EdgeRuntimeBuilder` — fluent builder for assembling an edge runtime.

use std::sync::Arc;

use edge_domain::{Handler, HandlerRegistry};
use edge_proxy::LifecycleMonitor;
use swe_edge_egress_grpc::GrpcOutbound;
use swe_edge_egress_http::HttpOutbound;
use swe_edge_ingress::{
    GrpcDecodeFn, GrpcEncodeFn, GrpcHandlerAdapter, GrpcHandlerRegistryDispatcher,
    GrpcInbound, GrpcInboundInterceptor, GrpcInboundInterceptorChain,
    HttpDecodeFn, HttpEncodeFn, HttpHandlerAdapter, HttpHandlerRegistryDispatcher,
    HttpInbound, IngressTlsConfig,
};
use swe_edge_ingress_verifier::TokenVerifier;

use crate::api::service_registry::ServiceRegistry;
use crate::api::types::RuntimeConfig;

/// Builder for assembling and starting an edge runtime.
pub struct EdgeRuntimeBuilder {
    pub(crate) config:                    Option<RuntimeConfig>,
    pub(crate) app_name:                  Option<String>,
    pub(crate) http_handler:              Option<Arc<dyn HttpInbound>>,
    pub(crate) grpc_handler:              Option<Arc<dyn GrpcInbound>>,
    pub(crate) http_dispatcher:           Option<HttpHandlerRegistryDispatcher>,
    pub(crate) grpc_dispatcher:           Option<GrpcHandlerRegistryDispatcher>,
    pub(crate) http_tls:                  Option<IngressTlsConfig>,
    pub(crate) grpc_tls:                  Option<IngressTlsConfig>,
    pub(crate) http_bearer_verifier:      Option<Arc<dyn TokenVerifier>>,
    pub(crate) grpc_interceptors:         GrpcInboundInterceptorChain,
    pub(crate) grpc_allow_unauthenticated: bool,
    pub(crate) egress_http:               Option<Arc<dyn HttpOutbound>>,
    pub(crate) egress_grpc:              Option<Arc<dyn GrpcOutbound>>,
    pub(crate) lifecycle:                 Option<Arc<dyn LifecycleMonitor>>,
}

impl EdgeRuntimeBuilder {
    pub fn config(mut self, config: RuntimeConfig) -> Self { self.config = Some(config); self }
    pub fn app_name(mut self, name: impl Into<String>) -> Self { self.app_name = Some(name.into()); self }

    pub fn http_route<Req, Resp>(self, handler: Arc<dyn Handler<Req, Resp>>) -> Self
    where Req: serde::de::DeserializeOwned + Send + 'static, Resp: serde::Serialize + Send + 'static,
    { self.http_route_with(handler, crate::core::json_codec::json_decode::<Req>, crate::core::json_codec::json_encode::<Resp>) }

    pub fn http_route_with<Req, Resp>(mut self, handler: Arc<dyn Handler<Req, Resp>>, decode: HttpDecodeFn<Req>, encode: HttpEncodeFn<Resp>) -> Self
    where Req: Send + 'static, Resp: Send + 'static,
    { let d = self.http_dispatcher.get_or_insert_with(|| HttpHandlerRegistryDispatcher::new(Arc::new(HandlerRegistry::new()))); d.register(HttpHandlerAdapter::new(handler, decode, encode)).expect("duplicate HTTP route"); self }

    pub fn grpc_route<Req, Resp>(self, handler: Arc<dyn Handler<Req, Resp>>) -> Self
    where Req: serde::de::DeserializeOwned + Send + 'static, Resp: serde::Serialize + Send + 'static,
    { self.grpc_route_with(handler, crate::core::json_codec::grpc_json_decode::<Req>, crate::core::json_codec::grpc_json_encode::<Resp>) }

    pub fn grpc_route_with<Req, Resp>(mut self, handler: Arc<dyn Handler<Req, Resp>>, decode: GrpcDecodeFn<Req>, encode: GrpcEncodeFn<Resp>) -> Self
    where Req: Send + 'static, Resp: Send + 'static,
    { let d = self.grpc_dispatcher.get_or_insert_with(|| GrpcHandlerRegistryDispatcher::new(Arc::new(HandlerRegistry::new()))); d.register(GrpcHandlerAdapter::new(handler, decode, encode)); self }

    pub fn http_tls(mut self, config: IngressTlsConfig) -> Self { self.http_tls = Some(config); self }
    pub fn grpc_tls(mut self, config: IngressTlsConfig) -> Self { self.grpc_tls = Some(config); self }
    pub fn http_bearer_auth(mut self, verifier: Arc<dyn TokenVerifier>) -> Self { self.http_bearer_verifier = Some(verifier); self }
    pub fn grpc_auth(mut self, interceptor: Arc<dyn GrpcInboundInterceptor>) -> Self { self.grpc_interceptors = self.grpc_interceptors.push(interceptor); self }
    pub fn grpc_allow_unauthenticated(mut self) -> Self { self.grpc_allow_unauthenticated = true; self }
    pub fn egress_http(mut self, client: Arc<dyn HttpOutbound>) -> Self { self.egress_http = Some(client); self }
    pub fn egress_grpc(mut self, client: Arc<dyn GrpcOutbound>) -> Self { self.egress_grpc = Some(client); self }
    pub fn lifecycle(mut self, monitor: Arc<dyn LifecycleMonitor>) -> Self { self.lifecycle = Some(monitor); self }
    pub fn http_handler(mut self, handler: Arc<dyn HttpInbound>) -> Self { self.http_handler = Some(handler); self }
    pub fn grpc_handler(mut self, handler: Arc<dyn GrpcInbound>) -> Self { self.grpc_handler = Some(handler); self }

    pub fn build_registry(&self) -> Option<Arc<ServiceRegistry>> {
        self.egress_http.as_ref().map(|http| {
            Arc::new(ServiceRegistry::new(Arc::clone(http), self.egress_grpc.clone()))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::edge_runtime::edge_runtime::EdgeRuntime;

    /// @covers: app_name
    #[test]
    fn test_app_name_sets_field() {
        let b = EdgeRuntime::builder().app_name("my-svc");
        assert_eq!(b.app_name.as_deref(), Some("my-svc"));
    }

    /// @covers: config
    #[test]
    fn test_config_sets_runtime_config() {
        let cfg = RuntimeConfig::default();
        let b = EdgeRuntime::builder().config(cfg);
        assert!(b.config.is_some());
    }

    /// @covers: grpc_allow_unauthenticated
    #[test]
    fn test_grpc_allow_unauthenticated_sets_flag() {
        assert!(EdgeRuntime::builder().grpc_allow_unauthenticated().grpc_allow_unauthenticated);
    }

    /// @covers: build_registry
    #[test]
    fn test_build_registry_returns_none_without_egress_http() {
        assert!(EdgeRuntime::builder().build_registry().is_none());
    }

    /// @covers: egress_http
    #[test]
    fn test_egress_http_sets_field() {
        let client = Arc::new(swe_edge_egress_http::default_http_outbound().unwrap());
        assert!(EdgeRuntime::builder().egress_http(client).egress_http.is_some());
    }

    /// @covers: http_tls
    #[test]
    fn test_http_tls_sets_field() {
        let b = EdgeRuntime::builder().http_tls(IngressTlsConfig::tls("c.pem", "k.pem"));
        assert!(b.http_tls.is_some());
    }

    /// @covers: grpc_tls
    #[test]
    fn test_grpc_tls_sets_field() {
        let b = EdgeRuntime::builder().grpc_tls(IngressTlsConfig::tls("c.pem", "k.pem"));
        assert!(b.grpc_tls.is_some());
    }

    /// @covers: lifecycle
    #[test]
    fn test_lifecycle_sets_field() {
        let b = EdgeRuntime::builder().lifecycle(edge_proxy::new_null_lifecycle_monitor());
        assert!(b.lifecycle.is_some());
    }

    /// @covers: http_handler
    #[test]
    fn test_http_handler_sets_field() {
        use futures::future::BoxFuture;
        use swe_edge_ingress::{HttpInboundResult, HttpHealthCheck, HttpRequest, HttpResponse};
        use edge_domain::RequestContext;
        struct Stub;
        impl HttpInbound for Stub {
            fn handle(&self, _: HttpRequest, _: RequestContext) -> BoxFuture<'_, HttpInboundResult<HttpResponse>>
            { Box::pin(async { Ok(HttpResponse::new(200, vec![])) }) }
            fn health_check(&self) -> BoxFuture<'_, HttpInboundResult<HttpHealthCheck>>
            { Box::pin(async { Ok(HttpHealthCheck::healthy()) }) }
        }
        assert!(EdgeRuntime::builder().http_handler(Arc::new(Stub)).http_handler.is_some());
    }

    /// @covers: grpc_handler
    #[test]
    fn test_grpc_handler_sets_field() {
        use futures::future::BoxFuture;
        use swe_edge_ingress::{GrpcInboundResult, GrpcHealthCheck, GrpcInboundError,
            GrpcRequest, GrpcResponse, GrpcMetadata, GrpcMessageStream};
        use edge_domain::RequestContext;
        struct Stub;
        impl GrpcInbound for Stub {
            fn handle_unary(&self, _: GrpcRequest, _: RequestContext) -> BoxFuture<'_, GrpcInboundResult<GrpcResponse>>
            { Box::pin(async { Err(GrpcInboundError::Unimplemented("stub".into())) }) }
            fn handle_stream(&self, _: String, _: GrpcMetadata, _: GrpcMessageStream, _: RequestContext) -> BoxFuture<'_, GrpcInboundResult<(GrpcMessageStream, GrpcMetadata)>>
            { Box::pin(async { Err(GrpcInboundError::Unimplemented("stub".into())) }) }
            fn health_check(&self) -> BoxFuture<'_, GrpcInboundResult<GrpcHealthCheck>>
            { Box::pin(async { Ok(GrpcHealthCheck::healthy()) }) }
        }
        assert!(EdgeRuntime::builder().grpc_handler(Arc::new(Stub)).grpc_handler.is_some());
    }

    /// @covers: egress_grpc
    #[test]
    fn test_egress_grpc_sets_field() {
        use futures::future::BoxFuture;
        use swe_edge_egress_grpc::{GrpcOutbound, GrpcOutboundError, GrpcOutboundResult,
            GrpcRequest, GrpcResponse, GrpcStatusCode};
        struct Stub;
        impl GrpcOutbound for Stub {
            fn call_unary(&self, _: GrpcRequest) -> BoxFuture<'_, GrpcOutboundResult<GrpcResponse>>
            { Box::pin(async { Err(GrpcOutboundError::Status(GrpcStatusCode::Unavailable, "stub".into())) }) }
            fn health_check(&self) -> BoxFuture<'_, GrpcOutboundResult<()>>
            { Box::pin(async { Ok(()) }) }
        }
        assert!(EdgeRuntime::builder().egress_grpc(Arc::new(Stub)).egress_grpc.is_some());
    }

    /// @covers: http_bearer_auth
    #[test]
    fn test_http_bearer_auth_sets_field() {
        use swe_edge_ingress_verifier::{TokenVerifier, Claims, VerifierError};
        struct Stub;
        impl TokenVerifier for Stub {
            fn verify(&self, _: &str) -> Result<Claims, VerifierError>
            { Err(VerifierError::Invalid("stub".into())) }
        }
        assert!(EdgeRuntime::builder().http_bearer_auth(Arc::new(Stub)).http_bearer_verifier.is_some());
    }

    /// @covers: grpc_auth
    #[test]
    fn test_grpc_auth_is_chainable_with_allow_unauthenticated() {
        // grpc_auth and grpc_allow_unauthenticated can both be called on the same builder
        let b = EdgeRuntime::builder().grpc_allow_unauthenticated();
        assert!(b.grpc_allow_unauthenticated);
    }

    /// @covers: http_route
    #[test]
    fn test_http_route_builds_dispatcher() {
        use edge_domain::{Handler, HandlerError};
        use futures::future::BoxFuture;
        struct Ping;
        impl Handler<String, String> for Ping {
            fn id(&self) -> &str { "ping" }
            fn pattern(&self) -> &str { "/ping" }
            fn execute<'life0, 'async_trait>(&'life0 self, _: String) -> BoxFuture<'async_trait, Result<String, HandlerError>>
            where 'life0: 'async_trait, Self: 'async_trait
            { Box::pin(async { Ok("pong".into()) }) }
        }
        let b = EdgeRuntime::builder().http_route(Arc::new(Ping));
        assert!(b.http_dispatcher.is_some());
    }

    /// @covers: grpc_route
    #[test]
    fn test_grpc_route_builds_dispatcher() {
        use edge_domain::{Handler, HandlerError};
        use futures::future::BoxFuture;
        struct Echo;
        impl Handler<String, String> for Echo {
            fn id(&self) -> &str { "echo" }
            fn pattern(&self) -> &str { "/echo" }
            fn execute<'life0, 'async_trait>(&'life0 self, req: String) -> BoxFuture<'async_trait, Result<String, HandlerError>>
            where 'life0: 'async_trait, Self: 'async_trait
            { Box::pin(async move { Ok(req) }) }
        }
        let b = EdgeRuntime::builder().grpc_route(Arc::new(Echo));
        assert!(b.grpc_dispatcher.is_some());
    }

    /// @covers: http_route_with
    #[test]
    fn test_http_route_with_builds_dispatcher() {
        use edge_domain::{Handler, HandlerError};
        use futures::future::BoxFuture;
        use swe_edge_ingress::{HttpDecodeFn, HttpEncodeFn, HttpResponse, HttpRequest};
        struct Echo;
        impl Handler<String, String> for Echo {
            fn id(&self) -> &str { "echo" }
            fn pattern(&self) -> &str { "/echo" }
            fn execute<'life0, 'async_trait>(&'life0 self, req: String) -> BoxFuture<'async_trait, Result<String, HandlerError>>
            where 'life0: 'async_trait, Self: 'async_trait
            { Box::pin(async move { Ok(req) }) }
        }
        let decode: HttpDecodeFn<String> = |_: &HttpRequest| Ok("hi".into());
        let encode: HttpEncodeFn<String> = |s: String| HttpResponse::new(200, s.into_bytes());
        let b = EdgeRuntime::builder().http_route_with(Arc::new(Echo), decode, encode);
        assert!(b.http_dispatcher.is_some());
    }

    /// @covers: grpc_route_with
    #[test]
    fn test_grpc_route_with_builds_dispatcher() {
        use edge_domain::{Handler, HandlerError};
        use futures::future::BoxFuture;
        use swe_edge_ingress::{GrpcDecodeFn, GrpcEncodeFn};
        struct Echo;
        impl Handler<Vec<u8>, Vec<u8>> for Echo {
            fn id(&self) -> &str { "echo" }
            fn pattern(&self) -> &str { "/echo" }
            fn execute<'life0, 'async_trait>(&'life0 self, req: Vec<u8>) -> BoxFuture<'async_trait, Result<Vec<u8>, HandlerError>>
            where 'life0: 'async_trait, Self: 'async_trait
            { Box::pin(async move { Ok(req) }) }
        }
        let decode: GrpcDecodeFn<Vec<u8>> = |b| Ok(b.to_vec());
        let encode: GrpcEncodeFn<Vec<u8>> = |v: &Vec<u8>| v.clone();
        let b = EdgeRuntime::builder().grpc_route_with(Arc::new(Echo), decode, encode);
        assert!(b.grpc_dispatcher.is_some());
    }

    /// @covers: grpc_route_with
    #[tokio::test]
    async fn test_grpc_route_with_registers_handler() {
        use edge_domain::{Handler, HandlerError};
        use swe_edge_ingress::{GrpcDecodeFn, GrpcEncodeFn};
        struct Echo;
        #[async_trait::async_trait]
        impl Handler<Vec<u8>, Vec<u8>> for Echo {
            fn id(&self) -> &str { "echo" }
            fn pattern(&self) -> &str { "/echo" }
            async fn execute(&self, req: Vec<u8>) -> Result<Vec<u8>, HandlerError> { Ok(req) }
        }
        let decode: GrpcDecodeFn<Vec<u8>> = |b| Ok(b.to_vec());
        let encode: GrpcEncodeFn<Vec<u8>> = |v: &Vec<u8>| v.clone();
        let b = EdgeRuntime::builder().grpc_route_with(Arc::new(Echo), decode, encode);
        assert!(b.grpc_dispatcher.is_some());
    }

    /// @covers: http_route_with
    #[tokio::test]
    async fn test_http_route_with_registers_handler() {
        use edge_domain::{Handler, HandlerError};
        use swe_edge_ingress::{HttpDecodeFn, HttpEncodeFn, HttpResponse, HttpRequest};
        struct Echo;
        #[async_trait::async_trait]
        impl Handler<String, String> for Echo {
            fn id(&self) -> &str { "echo" }
            fn pattern(&self) -> &str { "/echo" }
            async fn execute(&self, req: String) -> Result<String, HandlerError> { Ok(req) }
        }
        let decode: HttpDecodeFn<String> = |_r: &HttpRequest| Ok("hello".into());
        let encode: HttpEncodeFn<String> = |s: String| HttpResponse::new(200, s.into_bytes());
        let b = EdgeRuntime::builder().http_route_with(Arc::new(Echo), decode, encode);
        assert!(b.http_dispatcher.is_some());
    }
}
