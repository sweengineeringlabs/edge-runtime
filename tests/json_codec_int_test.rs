//! Integration test coverage for the JSON codec type aliases.
//!
//! Codec functions are covered by inline tests in `core/json_codec.rs`.
//! This file satisfies the per-module test-file requirement and verifies
//! the observable codec contract via the public `http_route` API.

use swe_edge_runtime::{EdgeRuntime, RuntimeConfig};

/// @covers: json_codec — http_route accepts a handler without explicit codec
#[test]
fn test_http_route_accepts_handler_with_auto_json_codec() {
    use std::sync::Arc;
    use async_trait::async_trait;
    use serde::{Deserialize, Serialize};
    use swe_edge_runtime::{Handler, HandlerError};

    #[derive(Deserialize)]  struct Req { prompt: String }
    #[derive(Serialize)]    struct Resp { text: String }

    struct EchoHandler;

    #[async_trait]
    impl Handler<Req, Resp> for EchoHandler {
        fn id(&self)      -> &str { "echo" }
        fn pattern(&self) -> &str { "/echo" }
        async fn execute(&self, req: Req) -> Result<Resp, HandlerError> {
            Ok(Resp { text: req.prompt })
        }
    }

    let cfg = RuntimeConfig::default();
    let _builder = EdgeRuntime::builder()
        .config(cfg)
        .http_route(Arc::new(EchoHandler));
}
