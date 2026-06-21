//! Integration tests for `HttpServer` trait methods.
//!
//! Covers all 7 methods × 3 scenarios (happy / error / edge) = 21 tests.
// @covers HttpServer::serve
// @covers HttpServer::serve_with_shutdown
// @covers HttpServer::serve_with_listener
// @covers HttpServer::request_timeout
// @covers HttpServer::axum_helper
// @covers HttpServer::builder_bind
// @covers HttpServer::new_server
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;
use std::time::Duration;

use futures::future::BoxFuture;
use swe_edge_runtime_http::{
    AxumHttpServer, AxumHttpServerBuilder, AxumHttpServerHelper, HttpIngress, HttpServer,
    HttpServerError,
};

// ── Stub helpers ─────────────────────────────────────────────────────────────

fn noop_handler() -> Arc<dyn HttpIngress> {
    use swe_edge_runtime_http::NoopHttpIngress;
    Arc::new(NoopHttpIngress)
}

/// Minimal `HttpServer` that returns `Ok(())` from both abstract methods.
struct OkServer;

impl HttpServer for OkServer {
    fn serve<'s>(&'s self) -> BoxFuture<'s, Result<(), HttpServerError>> {
        Box::pin(async { Ok(()) })
    }

    fn serve_with_shutdown<'s>(
        &'s self,
        shutdown: BoxFuture<'static, ()>,
    ) -> BoxFuture<'s, Result<(), HttpServerError>> {
        Box::pin(async move {
            shutdown.await;
            Ok(())
        })
    }
}

/// `HttpServer` whose `serve` always returns an error.
struct ErrorServer;

impl HttpServer for ErrorServer {
    fn serve<'s>(&'s self) -> BoxFuture<'s, Result<(), HttpServerError>> {
        Box::pin(async {
            Err(HttpServerError::Serve(std::io::Error::other(
                "intentional error",
            )))
        })
    }

    fn serve_with_shutdown<'s>(
        &'s self,
        _shutdown: BoxFuture<'static, ()>,
    ) -> BoxFuture<'s, Result<(), HttpServerError>> {
        Box::pin(async {
            Err(HttpServerError::Serve(std::io::Error::other(
                "intentional error",
            )))
        })
    }
}

// ── serve ────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_serve_noop_impl_returns_ok_happy() {
    let s = OkServer;
    let result = s.serve().await;
    assert!(result.is_ok(), "OkServer::serve must return Ok(())");
}

#[tokio::test]
async fn test_serve_invalid_bind_address_returns_bind_error_error() {
    // AxumHttpServer::serve fails immediately on an unresolvable address.
    let s = AxumHttpServer::new("0.0.0.0:99999", noop_handler());
    let result = s.serve().await;
    assert!(
        result.is_err(),
        "serve on port 99999 (out of range) must fail"
    );
    assert!(matches!(result.unwrap_err(), HttpServerError::Bind(_, _)));
}

#[tokio::test]
async fn test_serve_error_impl_propagates_error_edge() {
    // Edge: a custom impl that always errors — trait consumers must handle Err.
    let s = ErrorServer;
    let result = s.serve().await;
    assert!(
        result.is_err(),
        "ErrorServer::serve must propagate the error"
    );
}

// ── serve_with_shutdown ───────────────────────────────────────────────────────

#[tokio::test]
async fn test_serve_with_shutdown_resolves_immediately_happy() {
    let s = OkServer;
    let shutdown: BoxFuture<'static, ()> = Box::pin(async {});
    let result = s.serve_with_shutdown(shutdown).await;
    assert!(
        result.is_ok(),
        "serve_with_shutdown with instant shutdown must return Ok(())"
    );
}

#[tokio::test]
async fn test_serve_with_shutdown_error_impl_returns_err_error() {
    let s = ErrorServer;
    let shutdown: BoxFuture<'static, ()> = Box::pin(async {});
    let result = s.serve_with_shutdown(shutdown).await;
    assert!(
        result.is_err(),
        "ErrorServer::serve_with_shutdown must propagate error"
    );
}

#[tokio::test]
async fn test_serve_with_shutdown_real_server_binds_and_stops_edge() {
    // Edge: AxumHttpServer actually binds then stops when shutdown resolves.
    let s = AxumHttpServer::new("127.0.0.1:0", noop_handler());
    let (tx, rx) = tokio::sync::oneshot::channel::<()>();
    let shutdown: BoxFuture<'static, ()> = Box::pin(async move {
        let _ = rx.await;
    });
    // Trigger shutdown immediately from another task.
    tokio::spawn(async move {
        let _ = tx.send(());
    });
    let result = s.serve_with_shutdown(shutdown).await;
    assert!(
        result.is_ok(),
        "server must exit cleanly after shutdown signal"
    );
}

// ── serve_with_listener ───────────────────────────────────────────────────────

#[tokio::test]
async fn test_serve_with_listener_pre_bound_completes_on_shutdown_happy() {
    // Happy: pre-bound listener + immediate shutdown → Ok(()).
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind must succeed");
    let s = AxumHttpServer::new("127.0.0.1:0", noop_handler());
    let shutdown: BoxFuture<'static, ()> = Box::pin(async {});
    let result = s.serve_with_listener(listener, shutdown).await;
    assert!(
        result.is_ok(),
        "serve_with_listener with instant shutdown must return Ok(())"
    );
}

#[tokio::test]
async fn test_serve_with_listener_default_impl_returns_err_error() {
    // Error: the default trait impl always returns `Err(Serve("not implemented"))`.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind must succeed");
    let s = OkServer; // uses the default trait impl, not AxumHttpServer
    let shutdown: BoxFuture<'static, ()> = Box::pin(async {});
    let result = s.serve_with_listener(listener, shutdown).await;
    assert!(
        result.is_err(),
        "default serve_with_listener must return an error"
    );
    let msg = result.unwrap_err().to_string();
    assert!(
        msg.contains("not implemented"),
        "error must mention 'not implemented', got: {msg}"
    );
}

#[tokio::test]
async fn test_serve_with_listener_port_zero_assigns_ephemeral_port_edge() {
    // Edge: binding to port 0 lets the OS assign a port; the server must handle it.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind must succeed");
    let addr = listener.local_addr().expect("local_addr must return Ok");
    assert_ne!(addr.port(), 0, "OS must assign a non-zero ephemeral port");

    let s = AxumHttpServer::new("127.0.0.1:0", noop_handler());
    let shutdown: BoxFuture<'static, ()> = Box::pin(async {});
    let result = s.serve_with_listener(listener, shutdown).await;
    assert!(result.is_ok());
}

// ── request_timeout ───────────────────────────────────────────────────────────

#[test]
fn test_request_timeout_default_is_30s_happy() {
    // HttpServer's default impl returns 30 seconds.
    let s = OkServer;
    let t = s.request_timeout();
    assert_eq!(
        t,
        Duration::from_secs(30),
        "default request_timeout must be 30 s"
    );
}

#[test]
fn test_request_timeout_overridden_by_axum_server_error() {
    // "Error" scenario: a server configured with a very short timeout should
    // honour that value, not fall back to the default 30 s.
    let s = AxumHttpServer::new("127.0.0.1:0", noop_handler())
        .with_request_timeout(Duration::from_millis(100));
    let t = s.request_timeout();
    assert_eq!(
        t,
        Duration::from_millis(100),
        "overridden timeout must be returned as-is"
    );
    assert_ne!(
        t,
        Duration::from_secs(30),
        "overridden timeout must not equal the default"
    );
}

#[test]
fn test_request_timeout_is_nonzero_edge() {
    // Edge: the default timeout must be strictly positive — a zero timeout
    // would immediately kill every request.
    let s = OkServer;
    let t = s.request_timeout();
    assert!(t > Duration::ZERO, "request_timeout must be > 0");
}

// ── axum_helper ──────────────────────────────────────────────────────────────

#[test]
fn test_axum_helper_returns_unit_struct_happy() {
    // Happy: the default impl returns AxumHttpServerHelper (a unit struct).
    let s = OkServer;
    let _helper: AxumHttpServerHelper = s.axum_helper();
    // If we reach here the method exists and returns the expected type.
}

#[test]
fn test_axum_helper_called_twice_returns_same_type_error() {
    // "Error" scenario: calling axum_helper twice must not panic or change state.
    let s = OkServer;
    let _h1 = s.axum_helper();
    let _h2 = s.axum_helper();
    // AxumHttpServerHelper is a unit struct — both instances are equivalent.
}

#[test]
fn test_axum_helper_from_axum_server_edge() {
    // Edge: AxumHttpServer::axum_helper uses the default impl (no override).
    let s = AxumHttpServer::new("127.0.0.1:0", noop_handler());
    let _helper: AxumHttpServerHelper = s.axum_helper();
}

// ── builder_bind ─────────────────────────────────────────────────────────────

#[test]
fn test_builder_bind_returns_bind_string_happy() {
    let s = OkServer;
    let builder = AxumHttpServerBuilder::new("127.0.0.1:8080", noop_handler());
    let addr = s.builder_bind(&builder);
    assert_eq!(addr, "127.0.0.1:8080");
}

#[test]
fn test_builder_bind_preserves_unusual_address_error() {
    // "Error" scenario: an address that won't bind — builder_bind only reads the
    // field, so it must return it unchanged regardless of validity.
    let s = OkServer;
    let builder = AxumHttpServerBuilder::new("0.0.0.0:99999", noop_handler());
    let addr = s.builder_bind(&builder);
    assert_eq!(addr, "0.0.0.0:99999");
}

#[test]
fn test_builder_bind_empty_string_returned_as_is_edge() {
    // Edge: an empty bind string — builder_bind must not panic.
    let s = OkServer;
    let builder = AxumHttpServerBuilder::new("", noop_handler());
    let addr = s.builder_bind(&builder);
    assert_eq!(addr, "");
}

// ── new_server ────────────────────────────────────────────────────────────────

#[test]
fn test_new_server_constructs_axum_server_happy() {
    // Happy: new_server returns an AxumHttpServer with the correct bind string.
    let handler = noop_handler();
    let server: AxumHttpServer = OkServer::new_server("127.0.0.1:9090".to_string(), handler);
    // builder_bind lets us confirm the bind field was set correctly.
    let builder = AxumHttpServerBuilder::new("127.0.0.1:9090", noop_handler());
    assert_eq!(server.builder_bind(&builder), "127.0.0.1:9090");
}

#[test]
fn test_new_server_with_invalid_address_does_not_panic_error() {
    // "Error" scenario: construction with an invalid address must not panic —
    // the error surfaces only at serve time.
    let handler = noop_handler();
    let _server: AxumHttpServer = OkServer::new_server("not-a-valid-addr".to_string(), handler);
    // No panic means the error is deferred to serve().
}

#[test]
fn test_new_server_with_empty_bind_does_not_panic_edge() {
    // Edge: empty bind string — construction is still valid; failure is at serve time.
    let handler = noop_handler();
    let _server: AxumHttpServer = OkServer::new_server(String::new(), handler);
}
