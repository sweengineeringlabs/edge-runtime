//! Axum-based HTTP server implementation.

use axum::response::IntoResponse as _;
use axum::Router;
use futures::future::BoxFuture;
use tokio::net::TcpListener;
use tower::timeout::TimeoutLayer;
use tower::ServiceBuilder;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::trace::TraceLayer;

use edge_domain::SecurityContext;
use swe_edge_ingress_http::{
    HttpIngressError, HttpIngressResult, HttpRequest, HttpStream, SseStream, WsChannel,
};

use super::axum_http_server_helper::AxumHttpServerHelper;
use crate::api::AxumHttpServer;
use crate::api::HttpServer;
use crate::api::HttpServerError;

impl AxumHttpServer {
    /// Bind and serve until `shutdown` resolves.
    pub(crate) async fn serve<F>(&self, shutdown: F) -> Result<(), HttpServerError>
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        let listener = TcpListener::bind(&self.bind)
            .await
            .map_err(|e| HttpServerError::Bind(self.bind.clone(), e))?;
        self.dispatch(listener, shutdown).await
    }

    /// Serve using a caller-supplied pre-bound listener.
    pub(crate) async fn dispatch<F>(
        &self,
        listener: TcpListener,
        shutdown: F,
    ) -> Result<(), HttpServerError>
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        let bind_addr = listener
            .local_addr()
            .map(|a| a.to_string())
            .unwrap_or_else(|_| self.bind.clone());

        if let Some(ref tls_cfg) = self.tls {
            tracing::info!(bind = %bind_addr, mtls = tls_cfg.is_mtls(), "HTTPS server listening");
            AxumHttpServerHelper::serve_tls(
                listener,
                self.handler.clone(),
                self.body_limit,
                self.request_timeout,
                self.bearer_verifier.clone(),
                self.stream_handler.clone(),
                tls_cfg,
                shutdown,
            )
            .await
        } else {
            tracing::info!(bind = %bind_addr, "HTTP server listening");

            let handler = self.handler.clone();
            let body_limit = self.body_limit;
            let request_timeout = self.request_timeout;
            let verifier = self.bearer_verifier.clone();
            let stream_handler = self.stream_handler.clone();

            let app = Router::new()
                .fallback(move |req: axum::extract::Request| {
                    let handler = handler.clone();
                    let stream_handler = stream_handler.clone();
                    let verifier = verifier.clone();
                    async move {
                        let req = match AxumHttpServerHelper::verify_auth(req, verifier.as_deref())
                        {
                            Ok(r) => r,
                            Err(rsp) => return rsp,
                        };

                        if AxumHttpServerHelper::is_websocket_upgrade(req.headers()) {
                            if let Some(sh) = stream_handler {
                                return AxumHttpServerHelper::dispatch_websocket(req, sh).await;
                            }
                        }

                        if AxumHttpServerHelper::is_sse_request(req.headers()) {
                            if let Some(sh) = stream_handler {
                                return AxumHttpServerHelper::dispatch_sse(req, usize::MAX, sh)
                                    .await;
                            }
                        }

                        match AxumHttpServerHelper::extract_request(req, usize::MAX).await {
                            Ok((http_req, ctx)) => match handler.handle(http_req, ctx).await {
                                Ok(resp) => AxumHttpServerHelper::build_response(resp),
                                Err(e) => AxumHttpServerHelper::error_response(e),
                            },
                            Err(resp) => resp,
                        }
                    }
                })
                .layer(
                    ServiceBuilder::new()
                        .layer(axum::error_handling::HandleErrorLayer::new(
                            |e: axum::BoxError| async move {
                                if e.is::<tower::timeout::error::Elapsed>() {
                                    (axum::http::StatusCode::REQUEST_TIMEOUT, "request timed out")
                                        .into_response()
                                } else {
                                    (
                                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                        "internal server error",
                                    )
                                        .into_response()
                                }
                            },
                        ))
                        .layer(TraceLayer::new_for_http())
                        .layer(TimeoutLayer::new(request_timeout))
                        .layer(RequestBodyLimitLayer::new(body_limit)),
                );

            use axum::serve::ListenerExt as _;
            let listener = listener.tap_io(|stream| {
                stream
                    .set_nodelay(true)
                    .unwrap_or_else(|e| tracing::warn!("TCP_NODELAY: {e}"));
            });
            axum::serve(listener, app)
                .with_graceful_shutdown(shutdown)
                .await
                .map_err(HttpServerError::Serve)
        }
    }
}

impl HttpServer for AxumHttpServer {
    fn serve<'s>(&'s self) -> BoxFuture<'s, Result<(), HttpServerError>> {
        Box::pin(self.serve(futures::future::pending::<()>()))
    }

    fn serve_with_shutdown<'s>(
        &'s self,
        shutdown: BoxFuture<'static, ()>,
    ) -> BoxFuture<'s, Result<(), HttpServerError>> {
        Box::pin(self.serve(shutdown))
    }

    fn serve_with_listener<'s>(
        &'s self,
        listener: TcpListener,
        shutdown: BoxFuture<'static, ()>,
    ) -> BoxFuture<'s, Result<(), HttpServerError>> {
        Box::pin(self.dispatch(listener, shutdown))
    }

    fn request_timeout(&self) -> std::time::Duration {
        self.request_timeout
    }
}

impl HttpStream for AxumHttpServer {
    fn handle_sse(
        &self,
        _request: HttpRequest,
        _ctx: SecurityContext,
    ) -> BoxFuture<'_, HttpIngressResult<SseStream>> {
        Box::pin(async {
            Err(HttpIngressError::MethodNotAllowed(
                "SSE not configured on this server".to_string(),
            ))
        })
    }

    fn handle_websocket(
        &self,
        _request: HttpRequest,
        _ctx: SecurityContext,
        _channel: WsChannel,
    ) -> BoxFuture<'_, HttpIngressResult<()>> {
        Box::pin(async {
            Err(HttpIngressError::MethodNotAllowed(
                "WebSocket not configured on this server".to_string(),
            ))
        })
    }
}

#[cfg(test)]
mod dedicated_coverage {
    use super::AxumHttpServer;
    use futures::future::BoxFuture;
    use std::sync::Arc;
    use swe_edge_ingress_http::{
        HttpHealthCheck, HttpIngress, HttpIngressResult, HttpRequest, HttpResponse, MAX_BODY_BYTES,
    };
    use swe_edge_ingress_tls::IngressTlsConfig;

    fn make_handler() -> Arc<dyn HttpIngress> {
        struct AxumServerDispatcherOkIngress;
        impl HttpIngress for AxumServerDispatcherOkIngress {
            fn handle(
                &self,
                _: HttpRequest,
                _ctx: edge_domain::SecurityContext,
            ) -> BoxFuture<'_, HttpIngressResult<HttpResponse>> {
                Box::pin(async { Ok(HttpResponse::new(200, vec![])) })
            }
            fn health_check(&self) -> BoxFuture<'_, HttpIngressResult<HttpHealthCheck>> {
                Box::pin(async { Ok(HttpHealthCheck::healthy()) })
            }
        }
        Arc::new(AxumServerDispatcherOkIngress)
    }

    fn server() -> AxumHttpServer {
        AxumHttpServer::new("127.0.0.1:0", make_handler())
    }

    #[test]
    fn test_new_sets_default_body_limit() {
        let s = server();
        assert_eq!(s.body_limit, MAX_BODY_BYTES);
    }

    #[test]
    fn test_with_body_limit_overrides_default() {
        let s = server().with_body_limit(1024);
        assert_eq!(s.body_limit, 1024);
    }

    #[test]
    fn test_with_tls_sets_config() {
        let cfg = IngressTlsConfig::tls("cert.pem", "key.pem");
        let s = server().with_tls(cfg);
        assert!(s.tls.is_some());
        assert_eq!(
            s.body_limit, MAX_BODY_BYTES,
            "with_tls must not affect body_limit"
        );
    }

    #[tokio::test]
    async fn test_serve_bind_error_on_invalid_address() {
        let s = AxumHttpServer::new("0.0.0.0:99999", make_handler());
        let result = s.serve(std::future::ready(())).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_dispatch_completes_on_immediate_shutdown() {
        use tokio::net::TcpListener;
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let s = server();
        let result = s.dispatch(listener, std::future::ready(())).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_serve_is_constructible() {
        let s = server();
        assert_eq!(
            s.body_limit, MAX_BODY_BYTES,
            "server must have default body limit"
        );
    }

    #[test]
    fn test_dispatch_is_constructible() {
        let s = server();
        assert_eq!(
            s.body_limit, MAX_BODY_BYTES,
            "server must have default body limit"
        );
    }
}

#[cfg(test)]
mod sync_coverage {
    use super::AxumHttpServer;
    use futures::future::BoxFuture;
    use std::sync::Arc;
    use swe_edge_ingress_http::{
        HttpHealthCheck, HttpIngress, HttpIngressResult, HttpRequest, HttpResponse, MAX_BODY_BYTES,
    };

    fn make_handler() -> Arc<dyn HttpIngress> {
        struct AxumServerDispatcherOkIngress;
        impl HttpIngress for AxumServerDispatcherOkIngress {
            fn handle(
                &self,
                _: HttpRequest,
                _ctx: edge_domain::SecurityContext,
            ) -> BoxFuture<'_, HttpIngressResult<HttpResponse>> {
                Box::pin(async { Ok(HttpResponse::new(200, vec![])) })
            }
            fn health_check(&self) -> BoxFuture<'_, HttpIngressResult<HttpHealthCheck>> {
                Box::pin(async { Ok(HttpHealthCheck::healthy()) })
            }
        }
        Arc::new(AxumServerDispatcherOkIngress)
    }

    #[test]
    fn test_serve_type_exists() {
        let s = AxumHttpServer::new("127.0.0.1:0", make_handler());
        assert_eq!(
            s.body_limit, MAX_BODY_BYTES,
            "server must have default body limit"
        );
    }
}
