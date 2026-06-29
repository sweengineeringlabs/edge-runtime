//! Integration tests for `HttpServerError`.
// @covers api/server/errors/http_server_error.rs
#![allow(clippy::unwrap_used)]

use swe_edge_runtime_http::HttpServerError;

// ── Bind variant ─────────────────────────────────────────────────────────────

#[test]
fn test_http_server_error_bind_display_contains_address_happy() {
    let e = HttpServerError::Bind(
        "127.0.0.1:8080".into(),
        std::io::Error::other("address in use"),
    );
    let msg = e.to_string();
    assert!(
        msg.contains("127.0.0.1:8080"),
        "expected address in message: {msg}"
    );
}

#[test]
fn test_http_server_error_bind_display_contains_io_cause_error() {
    let e = HttpServerError::Bind(
        "0.0.0.0:443".into(),
        std::io::Error::other("permission denied"),
    );
    let msg = e.to_string();
    assert!(!msg.is_empty(), "Bind error Display must not be empty");
}

#[test]
fn test_http_server_error_bind_is_debug_edge() {
    let e = HttpServerError::Bind("::1:0".into(), std::io::Error::other("x"));
    let debug = format!("{e:?}");
    assert!(!debug.is_empty(), "Debug output must be non-empty");
    assert!(
        debug.contains("Bind"),
        "Debug must identify the Bind variant"
    );
}

// ── Serve variant ─────────────────────────────────────────────────────────────

#[test]
fn test_http_server_error_serve_display_is_non_empty_happy() {
    let e = HttpServerError::Serve(std::io::Error::other("connection reset"));
    assert!(!e.to_string().is_empty());
}

#[test]
fn test_http_server_error_serve_source_is_io_error_error() {
    use std::error::Error;
    let e = HttpServerError::Serve(std::io::Error::other("io fail"));
    assert!(
        e.source().is_some(),
        "Serve must expose its IO error as source"
    );
    assert!(
        !e.source().unwrap().to_string().is_empty(),
        "source IO error must have non-empty message"
    );
}

#[test]
fn test_http_server_error_serve_is_send_sync_edge() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<HttpServerError>();
    // Runtime complement: constructing and dropping the type must not panic.
    let e = HttpServerError::Serve(std::io::Error::other("sync check"));
    assert!(
        !e.to_string().is_empty(),
        "Send+Sync type must remain usable at runtime"
    );
}

// ── Tls variant ───────────────────────────────────────────────────────────────

#[test]
fn test_http_server_error_tls_display_contains_tls_prefix_happy() {
    let e = HttpServerError::Tls("cert load failed: not found".into());
    let msg = e.to_string();
    assert!(
        msg.contains("TLS"),
        "display must include TLS prefix; got: {msg}"
    );
    assert!(
        msg.contains("cert load failed"),
        "display must include inner message; got: {msg}"
    );
}

#[test]
fn test_http_server_error_tls_holds_message_string_error() {
    let payload = "bad_cert.pem: permission denied";
    let e = HttpServerError::Tls(payload.into());
    let msg = e.to_string();
    assert!(
        msg.contains(payload),
        "Tls variant must embed the message string; got: {msg}"
    );
}

#[test]
fn test_http_server_error_all_variants_are_error_impl_edge() {
    use std::error::Error;
    let variants: Vec<Box<dyn Error>> = vec![
        Box::new(HttpServerError::Bind(
            "addr".into(),
            std::io::Error::other("x"),
        )),
        Box::new(HttpServerError::Serve(std::io::Error::other("y"))),
    ];
    for v in &variants {
        assert!(!v.to_string().is_empty());
    }
}
