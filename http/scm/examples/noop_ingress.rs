//! Demonstrates constructing a [`NoopHttpIngress`] and calling its contract methods.

use futures::executor::block_on;
use swe_edge_runtime_http::{HttpIngress, HttpRequest, NoopHttpIngress};

fn main() {
    let ingress = NoopHttpIngress;
    let req = HttpRequest::get("/health");
    let ctx = edge_domain::SecurityContext::unauthenticated();

    match block_on(ingress.handle(&req, ctx)) {
        Ok(resp) => println!("status: {}", resp.status),
        Err(e) => eprintln!("handle error: {e}"),
    }

    match block_on(ingress.health_check()) {
        Ok(health) => println!("healthy: {}", health.healthy),
        Err(e) => eprintln!("health_check error: {e}"),
    }
}
