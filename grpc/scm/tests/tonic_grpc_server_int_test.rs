//! Wire integration tests for TonicGrpcServer.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::Duration;

use bytes::{BufMut, Bytes, BytesMut};
use http::Request;
use http_body_util::{BodyExt, Full};
use hyper_util::rt::{TokioExecutor, TokioIo};
use tokio::net::TcpListener;
use tokio::sync::oneshot;

use swe_edge_runtime_grpc::{GrpcServerManage, NoopGrpcIngress, TonicGrpcServer};

fn grpc_frame(payload: &[u8]) -> Bytes {
    let mut buf = BytesMut::with_capacity(5 + payload.len());
    buf.put_u8(0);
    buf.put_u32(payload.len() as u32);
    buf.put_slice(payload);
    buf.freeze()
}

/// @covers: TonicGrpcServer::serve_with_listener — basic round-trip
#[tokio::test]
async fn test_tonic_grpc_server_plaintext_round_trip_returns_grpc_ok() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let server =
        TonicGrpcServer::new("127.0.0.1:0", NoopGrpcIngress::create()).allow_unauthenticated(true);
    let (tx, rx) = oneshot::channel::<()>();
    tokio::spawn(async move {
        server
            .serve_with_listener(listener, async move {
                let _ = rx.await;
            })
            .await
            .unwrap();
    });
    tokio::time::sleep(Duration::from_millis(30)).await;

    let stream = tokio::net::TcpStream::connect(addr).await.unwrap();
    let io = TokioIo::new(stream);
    let (mut sender, conn) = hyper::client::conn::http2::Builder::new(TokioExecutor::new())
        .handshake(io)
        .await
        .unwrap();
    tokio::spawn(conn);

    let req = Request::builder()
        .method("POST")
        .uri(format!("http://{addr}/pkg.Svc/Echo"))
        .header("content-type", "application/grpc")
        .header("te", "trailers")
        .body(Full::new(grpc_frame(b"hello")))
        .unwrap();
    let resp = sender.send_request(req).await.unwrap();
    let collected = resp.into_body().collect().await.unwrap();
    let trailers = collected.trailers().unwrap();
    assert_eq!(
        trailers.get("grpc-status").and_then(|v| v.to_str().ok()),
        Some("0")
    );
    let _ = tx.send(());
}

/// @covers: TonicGrpcServer::serve — bind error on invalid address
#[tokio::test]
async fn test_tonic_grpc_server_serve_returns_error_on_invalid_bind() {
    let s = TonicGrpcServer::new("0.0.0.0:99999", NoopGrpcIngress::create())
        .allow_unauthenticated(true);
    assert!(s.serve(std::future::ready(())).await.is_err());
}

/// @covers: TonicGrpcServer::serve_with_listener — immediate shutdown
#[tokio::test]
async fn test_tonic_grpc_server_serve_with_listener_completes_on_immediate_shutdown() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let s =
        TonicGrpcServer::new("127.0.0.1:0", NoopGrpcIngress::create()).allow_unauthenticated(true);
    let result = s
        .serve_with_listener(listener, std::future::ready(()))
        .await;
    assert!(
        result.is_ok(),
        "serve_with_listener must return Ok on immediate shutdown"
    );
    // The server was not mutated during serve — reflection remains off.
    assert!(
        !s.is_reflection_enabled(),
        "reflection must still be off after serve returns"
    );
}
