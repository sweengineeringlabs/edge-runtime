//! Example: using NoopGrpcIngress as a stand-in during development.
//!
//! Run: cargo run --example noop_ingress

use edge_domain::SecurityContext;
use futures::executor::block_on;
use swe_edge_runtime_grpc::{GrpcIngress, GrpcRequest, NoopGrpcIngress};

fn main() {
    let handler = NoopGrpcIngress::create();

    let req = GrpcRequest::new("acme.Greeter", "SayHello", b"hello".to_vec())
        .with_metadata("x-request-id", "req-001");

    let ctx = SecurityContext::unauthenticated();

    match block_on(handler.handle_unary(&req, ctx)) {
        Ok(resp) => {
            println!("status: {}", resp.status);
            println!("body_len: {}", resp.body.len());
            println!("is_ok: {}", resp.is_ok());
        }
        Err(e) => eprintln!("handle_unary error: {e}"),
    }

    match block_on(handler.health_check()) {
        Ok(health) => println!("healthy: {}", health.healthy),
        Err(e) => eprintln!("health_check error: {e}"),
    }
}
