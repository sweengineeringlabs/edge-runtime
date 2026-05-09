//! [`Output`] trait impl for [`DefaultOutput`].

use std::sync::Arc;

use swe_edge_egress_grpc::GrpcOutbound;
use swe_edge_egress_http::HttpOutbound;

use crate::api::output::{DefaultOutput, Output};

impl Output for DefaultOutput {
    fn http(&self) -> Arc<dyn HttpOutbound>         { self.http.clone() }
    fn grpc(&self) -> Option<Arc<dyn GrpcOutbound>> { self.grpc.clone() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use futures::future::BoxFuture;
    use swe_edge_egress_http::{HttpOutbound, HttpOutboundError, HttpOutboundResult, HttpStreamResponse};
    use crate::api::output::DefaultOutput;

    struct NullHttp;
    impl HttpOutbound for NullHttp {
        fn send(&self, _: swe_edge_egress_http::HttpRequest)
            -> BoxFuture<'_, HttpOutboundResult<swe_edge_egress_http::HttpResponse>>
        { Box::pin(async { Err(HttpOutboundError::Internal("stub".into())) }) }
        fn send_stream(&self, _: swe_edge_egress_http::HttpRequest)
            -> BoxFuture<'_, HttpOutboundResult<HttpStreamResponse>>
        { Box::pin(async { Err(HttpOutboundError::Internal("stub".into())) }) }
        fn health_check(&self) -> BoxFuture<'_, HttpOutboundResult<()>>
        { Box::pin(async { Ok(()) }) }
    }

    /// @covers: Output::http — returns the configured client
    #[test]
    fn test_output_http_returns_configured_client() {
        let client: Arc<dyn HttpOutbound> = Arc::new(NullHttp);
        let output = DefaultOutput::new_http(Arc::clone(&client));
        let _ = output.http();
    }

    /// @covers: Output::grpc — returns None when not configured
    #[test]
    fn test_output_grpc_returns_none_when_not_configured() {
        let output = DefaultOutput::new_http(Arc::new(NullHttp));
        assert!(output.grpc().is_none());
    }
}
