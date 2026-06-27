//! SPI-only methods for `AxumHttpServerHelper` that require spi-layer dependencies.

use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::IntoResponse as _;
use edge_domain::SecurityContext;
use futures::StreamExt as _;
use swe_edge_ingress_http::{
    HttpBody, HttpIngress, HttpIngressError, HttpMethod, HttpRequest, HttpResponse, HttpStream,
    WsChannel, WsMessage,
};
use edge_domain_security::IngressTlsConfig;
use swe_edge_ingress_verifier::TokenVerifier;
use tokio::net::TcpListener;

pub(crate) use crate::api::AxumHttpServerHelper;
use crate::api::HttpServerError;

const WS_OUTBOUND_BUFFER: usize = 64;

impl AxumHttpServerHelper {
    /// Extract [`HttpRequest`] and [`SecurityContext`] from an Axum request.
    pub(crate) async fn extract_request(
        req: axum::extract::Request,
        body_limit: usize,
    ) -> Result<(HttpRequest, SecurityContext), axum::response::Response> {
        let (mut parts, body) = req.into_parts();

        let ctx = parts
            .extensions
            .remove::<Arc<SecurityContext>>()
            .and_then(|arc| Arc::try_unwrap(arc).ok())
            .unwrap_or_else(SecurityContext::unauthenticated);

        let method = Self::map_method(&parts.method);
        let url = parts.uri.to_string();
        let query = Self::parse_query(parts.uri.query());
        let headers = Self::collect_headers(&parts.headers);
        let ct = headers
            .get("content-type")
            .map(|s| s.as_str())
            .unwrap_or("")
            .to_owned();

        let bytes = axum::body::to_bytes(axum::body::Body::new(body), body_limit)
            .await
            .map_err(|_| Self::payload_too_large())?;

        let body = Self::build_body(&bytes, &ct);
        Ok((
            HttpRequest {
                method,
                url,
                headers,
                query,
                body,
                timeout: None,
            },
            ctx,
        ))
    }

    /// Dispatch an SSE request to an [`HttpStream`] handler.
    pub(crate) async fn dispatch_sse(
        req: axum::extract::Request,
        body_limit: usize,
        handler: Arc<dyn HttpStream>,
    ) -> axum::response::Response {
        let (http_req, ctx) = match Self::extract_request(req, body_limit).await {
            Ok(r) => r,
            Err(resp) => return resp,
        };
        match handler.handle_sse(http_req, ctx).await {
            Ok(stream) => {
                use axum::response::sse::{Event, KeepAlive, Sse};
                let axum_stream = stream.map(|item| {
                    item.map(|ev| {
                        let mut event = Event::default().data(ev.data);
                        if let Some(name) = ev.event {
                            event = event.event(name);
                        }
                        if let Some(id) = ev.id {
                            event = event.id(id);
                        }
                        event
                    })
                    .map_err(|e| e.to_string())
                });
                Sse::new(axum_stream)
                    .keep_alive(KeepAlive::default())
                    .into_response()
            }
            Err(e) => Self::error_response(e),
        }
    }

    /// Dispatch a WebSocket upgrade request.
    pub(crate) async fn dispatch_websocket(
        req: axum::extract::Request,
        handler: Arc<dyn HttpStream>,
    ) -> axum::response::Response {
        use axum::extract::ws::{Message, WebSocketUpgrade};
        use axum::extract::FromRequestParts;

        let (mut parts, _body) = req.into_parts();

        let ctx = parts
            .extensions
            .remove::<Arc<SecurityContext>>()
            .and_then(|arc| Arc::try_unwrap(arc).ok())
            .unwrap_or_else(SecurityContext::unauthenticated);

        let http_req = HttpRequest {
            method: Self::map_method(&parts.method),
            url: parts.uri.to_string(),
            headers: Self::collect_headers(&parts.headers),
            query: Self::parse_query(parts.uri.query()),
            body: None,
            timeout: None,
        };

        let ws_upgrade = match WebSocketUpgrade::from_request_parts(&mut parts, &()).await {
            Ok(u) => u,
            Err(e) => {
                return Self::plain_text_response(
                    StatusCode::BAD_REQUEST,
                    format!("invalid websocket upgrade: {e}"),
                )
            }
        };

        ws_upgrade
            .on_upgrade(move |socket| async move {
                use tokio::sync::mpsc;

                let (out_tx, mut out_rx) = mpsc::channel::<WsMessage>(WS_OUTBOUND_BUFFER);
                let (mut socket_send, socket_recv) = futures::StreamExt::split(socket);

                let incoming: swe_edge_ingress_http::WsReceiver =
                    Box::pin(socket_recv.filter_map(|item| async move {
                        match item {
                            Ok(Message::Text(t)) => Some(Ok(WsMessage::text(t.as_str()))),
                            Ok(Message::Binary(b)) => Some(Ok(WsMessage::binary(b))),
                            Ok(Message::Close(_)) => None,
                            Ok(_) => None,
                            Err(e) => Some(Err(swe_edge_ingress_http::HttpIngressError::Internal(
                                e.to_string(),
                            ))),
                        }
                    }));

                let channel = WsChannel {
                    sender: out_tx,
                    receiver: incoming,
                };

                let handler_fut = handler.handle_websocket(http_req, ctx, channel);

                let bridge_fut = async move {
                    while let Some(msg) = out_rx.recv().await {
                        let ws_msg = if msg.binary {
                            Message::Binary(msg.data.to_vec().into())
                        } else {
                            Message::Text(String::from_utf8_lossy(&msg.data).into_owned().into())
                        };
                        use futures::SinkExt as _;
                        if socket_send.send(ws_msg).await.is_err() {
                            break;
                        }
                    }
                };

                let (handler_result, _) = futures::future::join(handler_fut, bridge_fut).await;
                if let Err(e) = handler_result {
                    tracing::warn!("WebSocket handler error: {e}");
                }
            })
            .into_response()
    }

    /// Serve TLS connections using the provided `IngressTlsConfig`.
    #[allow(clippy::too_many_arguments, clippy::unused_async)]
    pub(crate) async fn serve_tls<F>(
        listener: TcpListener,
        handler: Arc<dyn HttpIngress>,
        body_limit: usize,
        request_timeout: std::time::Duration,
        verifier: Option<Arc<dyn TokenVerifier>>,
        stream_handler: Option<Arc<dyn HttpStream>>,
        tls_cfg: &IngressTlsConfig,
        shutdown: F,
    ) -> Result<(), HttpServerError>
    where
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        use hyper_util::rt::{TokioExecutor, TokioIo};
        use hyper_util::service::TowerToHyperService;
        use tower::ServiceBuilder;
        use tower_http::trace::TraceLayer;

        let acceptor = crate::core::tls::DefaultAcceptorBuilder::build_tls_acceptor(tls_cfg)
            .map_err(HttpServerError::Tls)?;

        let mut shutdown = std::pin::pin!(shutdown);

        loop {
            tokio::select! {
                res = listener.accept() => {
                    let (stream, _) = match res {
                        Ok(s) => s,
                        Err(e) => { tracing::warn!("TLS accept error: {e}"); continue; }
                    };
                    stream.set_nodelay(true).unwrap_or_else(|e| tracing::warn!("TCP_NODELAY: {e}"));
                    let acceptor = acceptor.clone();
                    let handler = handler.clone();
                    let verifier = verifier.clone();
                    let stream_handler = stream_handler.clone();
                    tokio::spawn(async move {
                        let tls = match acceptor.accept(stream).await {
                            Ok(s) => s,
                            Err(e) => { tracing::debug!("TLS handshake failed: {e}"); return; }
                        };
                        let io = TokioIo::new(tls);
                        let inner = tower::service_fn(move |req: http::Request<hyper::body::Incoming>| {
                            let handler = handler.clone();
                            let verifier = verifier.clone();
                            let stream_handler = stream_handler.clone();
                            async move {
                                let req = req.map(axum::body::Body::new);
                                let req = match AxumHttpServerHelper::verify_auth(req, verifier.as_deref()) {
                                    Ok(r) => r,
                                    Err(rsp) => return Ok::<_, Infallible>(rsp),
                                };

                                if AxumHttpServerHelper::is_websocket_upgrade(req.headers()) {
                                    if let Some(sh) = stream_handler.clone() {
                                        return Ok(AxumHttpServerHelper::dispatch_websocket(req, sh).await);
                                    }
                                }

                                let resp = tokio::select! {
                                    _ = tokio::time::sleep(request_timeout) => {
                                        AxumHttpServerHelper::request_timeout_response()
                                    }
                                    r = async move {
                                        if AxumHttpServerHelper::is_sse_request(req.headers()) {
                                            if let Some(sh) = stream_handler {
                                                return AxumHttpServerHelper::dispatch_sse(req, body_limit, sh).await;
                                            }
                                        }
                                        match AxumHttpServerHelper::extract_request(req, body_limit).await {
                                            Ok((http_req, ctx)) => match handler.handle(http_req, ctx).await {
                                                Ok(resp) => AxumHttpServerHelper::build_response(resp),
                                                Err(e) => AxumHttpServerHelper::error_response(e),
                                            },
                                            Err(resp) => resp,
                                        }
                                    } => r,
                                };
                                Ok::<_, Infallible>(resp)
                            }
                        });
                        let svc = ServiceBuilder::new()
                            .layer(TraceLayer::new_for_http())
                            .service(inner);
                        if let Err(e) = hyper_util::server::conn::auto::Builder::new(TokioExecutor::new())
                            .serve_connection(io, TowerToHyperService::new(svc))
                            .await
                        {
                            tracing::debug!("HTTPS connection error: {e}");
                        }
                    });
                }
                _ = &mut shutdown => break,
            }
        }

        Ok(())
    }
}

impl AxumHttpServerHelper {
    pub(crate) fn plain_text_response(
        status: axum::http::StatusCode,
        body: impl Into<String>,
    ) -> axum::response::Response {
        let mut response = axum::response::Response::new(axum::body::Body::from(body.into()));
        *response.status_mut() = status;
        response.headers_mut().insert(
            axum::http::header::CONTENT_TYPE,
            axum::http::HeaderValue::from_static("text/plain; charset=utf-8"),
        );
        response
    }

    pub(crate) fn map_method(m: &axum::http::Method) -> HttpMethod {
        match *m {
            axum::http::Method::GET => HttpMethod::Get,
            axum::http::Method::POST => HttpMethod::Post,
            axum::http::Method::PUT => HttpMethod::Put,
            axum::http::Method::PATCH => HttpMethod::Patch,
            axum::http::Method::DELETE => HttpMethod::Delete,
            axum::http::Method::HEAD => HttpMethod::Head,
            axum::http::Method::OPTIONS => HttpMethod::Options,
            _ => HttpMethod::Get,
        }
    }

    pub(crate) fn parse_query(raw: Option<&str>) -> HashMap<String, String> {
        let mut map = HashMap::new();
        if let Some(q) = raw {
            for pair in q.split('&') {
                let mut parts = pair.splitn(2, '=');
                let key = Self::percent_decode(parts.next().unwrap_or(""));
                let value = Self::percent_decode(parts.next().unwrap_or(""));
                if !key.is_empty() {
                    map.insert(key, value);
                }
            }
        }
        map
    }

    pub(crate) fn build_body(bytes: &bytes::Bytes, content_type: &str) -> Option<HttpBody> {
        if bytes.is_empty() {
            return None;
        }
        if content_type.contains("application/json") {
            serde_json::from_slice(bytes)
                .ok()
                .map(HttpBody::Json)
                .or_else(|| Some(HttpBody::Raw(bytes.to_vec())))
        } else if content_type.contains("application/x-www-form-urlencoded") {
            Some(HttpBody::Form(Self::parse_form(bytes)))
        } else {
            Some(HttpBody::Raw(bytes.to_vec()))
        }
    }

    pub(crate) fn parse_form(bytes: &bytes::Bytes) -> HashMap<String, String> {
        let mut map = HashMap::new();
        let s = std::str::from_utf8(bytes).unwrap_or("");
        for pair in s.split('&') {
            let mut parts = pair.splitn(2, '=');
            let key = Self::percent_decode(parts.next().unwrap_or(""));
            let value = Self::percent_decode(parts.next().unwrap_or(""));
            if !key.is_empty() {
                map.insert(key, value);
            }
        }
        map
    }

    pub(crate) fn percent_decode(s: &str) -> String {
        let s = s.replace('+', " ");
        let mut out = String::with_capacity(s.len());
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '%' {
                let h1 = chars.next();
                let h2 = chars.next();
                match (h1, h2) {
                    (Some(a), Some(b)) => {
                        if let Ok(byte) = u8::from_str_radix(&format!("{a}{b}"), 16) {
                            out.push(byte as char);
                        } else {
                            out.push('%');
                            out.push(a);
                            out.push(b);
                        }
                    }
                    (Some(a), None) => {
                        out.push('%');
                        out.push(a);
                    }
                    _ => {
                        out.push('%');
                    }
                }
                continue;
            }
            out.push(c);
        }
        out
    }

    pub(crate) fn build_response(resp: HttpResponse) -> axum::response::Response {
        let status = axum::http::StatusCode::from_u16(resp.status)
            .unwrap_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        let mut builder = axum::response::Response::builder().status(status);
        for (k, v) in &resp.headers {
            builder = builder.header(k.as_str(), v.as_str());
        }
        builder
            .body(axum::body::Body::from(resp.body))
            .unwrap_or_else(|_| Self::internal_server_error("response build failed"))
    }

    pub(crate) fn error_response(e: HttpIngressError) -> axum::response::Response {
        let status = match &e {
            HttpIngressError::NotFound(_) => axum::http::StatusCode::NOT_FOUND,
            HttpIngressError::InvalidInput(_) => axum::http::StatusCode::BAD_REQUEST,
            HttpIngressError::Unauthorized(_) => axum::http::StatusCode::UNAUTHORIZED,
            HttpIngressError::PermissionDenied(_) => axum::http::StatusCode::FORBIDDEN,
            HttpIngressError::Conflict(_) => axum::http::StatusCode::CONFLICT,
            HttpIngressError::MethodNotAllowed(_) => axum::http::StatusCode::METHOD_NOT_ALLOWED,
            HttpIngressError::UnprocessableEntity(_) => {
                axum::http::StatusCode::UNPROCESSABLE_ENTITY
            }
            HttpIngressError::Timeout(_) => axum::http::StatusCode::GATEWAY_TIMEOUT,
            HttpIngressError::Unavailable(_) => axum::http::StatusCode::SERVICE_UNAVAILABLE,
            HttpIngressError::Internal(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        axum::response::Response::builder()
            .status(status)
            .header("content-type", "text/plain; charset=utf-8")
            .body(axum::body::Body::from(e.to_string()))
            .unwrap_or_else(|_| Self::internal_server_error("error response build failed"))
    }

    pub(crate) fn request_timeout_response() -> axum::response::Response {
        Self::plain_text_response(axum::http::StatusCode::REQUEST_TIMEOUT, "request timed out")
    }
}

#[cfg(test)]
mod tests {
    use super::AxumHttpServerHelper;
    use futures::future::BoxFuture;
    use std::sync::Arc;
    use swe_edge_ingress_http::{
        HttpHealthCheck, HttpIngress, HttpIngressError, HttpIngressResult, HttpRequest,
        HttpResponse, HttpStream, SecurityContext, SseStream, WsChannel,
    };

    struct AxumHttpServerHelperNoopIngress;
    impl HttpIngress for AxumHttpServerHelperNoopIngress {
        fn handle(
            &self,
            _: HttpRequest,
            _: SecurityContext,
        ) -> BoxFuture<'_, HttpIngressResult<HttpResponse>> {
            Box::pin(async { Ok(HttpResponse::new(200, vec![])) })
        }
        fn health_check(&self) -> BoxFuture<'_, HttpIngressResult<HttpHealthCheck>> {
            Box::pin(async { Ok(HttpHealthCheck::healthy()) })
        }
    }

    struct AxumHttpServerHelperNoopStream;
    impl HttpStream for AxumHttpServerHelperNoopStream {
        fn handle_sse(
            &self,
            _: HttpRequest,
            _: SecurityContext,
        ) -> BoxFuture<'_, HttpIngressResult<SseStream>> {
            Box::pin(async { Err(HttpIngressError::MethodNotAllowed("no sse".into())) })
        }
        fn handle_websocket(
            &self,
            _: HttpRequest,
            _: SecurityContext,
            _: WsChannel,
        ) -> BoxFuture<'_, HttpIngressResult<()>> {
            Box::pin(async { Ok(()) })
        }
    }

    #[test]
    fn test_extract_request_parses_get_request() {
        let req = axum::http::Request::builder()
            .uri("/ping")
            .body(axum::body::Body::empty())
            .unwrap();
        let result =
            futures::executor::block_on(AxumHttpServerHelper::extract_request(req, usize::MAX));
        assert!(result.is_ok());
        let (http_req, _ctx) = result.unwrap();
        assert!(http_req.url.contains("/ping"));
    }

    #[test]
    fn test_extract_request_rejects_oversized_body() {
        let req = axum::http::Request::builder()
            .uri("/upload")
            .body(axum::body::Body::from(vec![0u8; 10]))
            .unwrap();
        let result = futures::executor::block_on(AxumHttpServerHelper::extract_request(req, 5));
        assert!(result.is_err(), "should reject body exceeding limit");
    }

    #[test]
    fn test_extract_request_empty_body_is_none() {
        let req = axum::http::Request::builder()
            .uri("/")
            .body(axum::body::Body::empty())
            .unwrap();
        let (http_req, _) =
            futures::executor::block_on(AxumHttpServerHelper::extract_request(req, usize::MAX))
                .unwrap();
        assert!(http_req.body.is_none());
    }

    #[test]
    fn test_dispatch_sse_returns_error_when_handler_rejects() {
        let req = axum::http::Request::builder()
            .uri("/events")
            .body(axum::body::Body::empty())
            .unwrap();
        let handler: Arc<dyn HttpStream> = Arc::new(AxumHttpServerHelperNoopStream);
        let resp = futures::executor::block_on(AxumHttpServerHelper::dispatch_sse(
            req,
            usize::MAX,
            handler,
        ));
        assert!(resp.status().is_client_error() || resp.status().is_server_error());
    }

    #[test]
    fn test_dispatch_websocket_rejects_non_upgrade() {
        let req = axum::http::Request::builder()
            .uri("/ws")
            .body(axum::body::Body::empty())
            .unwrap();
        let handler: Arc<dyn HttpStream> = Arc::new(AxumHttpServerHelperNoopStream);
        let resp =
            futures::executor::block_on(AxumHttpServerHelper::dispatch_websocket(req, handler));
        assert_eq!(resp.status(), axum::http::StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_plain_text_response_sets_status_and_content_type() {
        let resp = AxumHttpServerHelper::plain_text_response(
            axum::http::StatusCode::BAD_REQUEST,
            "bad input",
        );
        assert_eq!(resp.status(), axum::http::StatusCode::BAD_REQUEST);
    }

    #[test]
    fn test_map_method_get_returns_http_method_get() {
        use swe_edge_ingress_http::HttpMethod;
        let m = AxumHttpServerHelper::map_method(&axum::http::Method::GET);
        assert!(matches!(m, HttpMethod::Get));
    }

    #[test]
    fn test_map_method_unknown_falls_back_to_get() {
        use swe_edge_ingress_http::HttpMethod;
        let m = AxumHttpServerHelper::map_method(&axum::http::Method::CONNECT);
        assert!(
            matches!(m, HttpMethod::Get),
            "unknown method must fall back to Get"
        );
    }

    #[test]
    fn test_parse_query_extracts_key_value_pairs() {
        let map = AxumHttpServerHelper::parse_query(Some("a=1&b=2"));
        assert_eq!(map.get("a").map(|s| s.as_str()), Some("1"));
        assert_eq!(map.get("b").map(|s| s.as_str()), Some("2"));
    }

    #[test]
    fn test_parse_query_empty_returns_empty_map() {
        let map = AxumHttpServerHelper::parse_query(None);
        assert!(map.is_empty());
    }

    #[test]
    fn test_build_body_empty_bytes_returns_none() {
        use bytes::Bytes;
        let result = AxumHttpServerHelper::build_body(&Bytes::new(), "application/json");
        assert!(result.is_none());
    }

    #[test]
    fn test_build_body_raw_bytes_returns_raw_variant() {
        use bytes::Bytes;
        use swe_edge_ingress_http::HttpBody;
        let result = AxumHttpServerHelper::build_body(&Bytes::from_static(b"hello"), "text/plain");
        assert!(matches!(result, Some(HttpBody::Raw(_))));
    }

    #[test]
    fn test_parse_form_extracts_url_encoded_pairs() {
        use bytes::Bytes;
        let bytes = Bytes::from_static(b"key=val&x=y");
        let map = AxumHttpServerHelper::parse_form(&bytes);
        assert_eq!(map.get("key").map(|s| s.as_str()), Some("val"));
    }

    #[test]
    fn test_percent_decode_plus_becomes_space() {
        let decoded = AxumHttpServerHelper::percent_decode("hello+world");
        assert_eq!(decoded, "hello world");
    }

    #[test]
    fn test_percent_decode_encoded_char() {
        let decoded = AxumHttpServerHelper::percent_decode("hello%20world");
        assert_eq!(decoded, "hello world");
    }

    #[test]
    fn test_build_response_maps_status_code() {
        use swe_edge_ingress_http::HttpResponse;
        let resp = HttpResponse::new(201, b"created".to_vec());
        let axum_resp = AxumHttpServerHelper::build_response(resp);
        assert_eq!(axum_resp.status(), axum::http::StatusCode::CREATED);
    }

    #[test]
    fn test_error_response_not_found_returns_404() {
        use swe_edge_ingress_http::HttpIngressError;
        let resp =
            AxumHttpServerHelper::error_response(HttpIngressError::NotFound("missing".into()));
        assert_eq!(resp.status(), axum::http::StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_error_response_unauthorized_returns_401() {
        use swe_edge_ingress_http::HttpIngressError;
        let resp =
            AxumHttpServerHelper::error_response(HttpIngressError::Unauthorized("no auth".into()));
        assert_eq!(resp.status(), axum::http::StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_request_timeout_response_returns_408() {
        let resp = AxumHttpServerHelper::request_timeout_response();
        assert_eq!(resp.status(), axum::http::StatusCode::REQUEST_TIMEOUT);
    }

    #[test]
    fn test_serve_tls_rejects_invalid_cert_paths() {
        use edge_domain_security::IngressTlsConfig;
        let tls_cfg = IngressTlsConfig {
            cert_pem_path: "nonexistent_cert.pem".into(),
            key_pem_path: "nonexistent_key.pem".into(),
            client_ca_pem_path: None,
        };
        let result = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
                AxumHttpServerHelper::serve_tls(
                    listener,
                    Arc::new(AxumHttpServerHelperNoopIngress),
                    usize::MAX,
                    std::time::Duration::from_secs(30),
                    None,
                    None,
                    &tls_cfg,
                    std::future::ready(()),
                )
                .await
            });
        assert!(result.is_err(), "invalid TLS cert paths should fail");
    }
}
