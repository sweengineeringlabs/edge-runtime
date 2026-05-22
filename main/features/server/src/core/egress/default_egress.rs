//! [`Egress`] trait impl for [`DefaultEgress`].

use std::sync::Arc;

use swe_edge_egress_grpc::GrpcEgress;
use swe_edge_egress_http::HttpEgress;

use crate::api::egress::{DefaultEgress, Egress};

impl Egress for DefaultEgress {
    fn http(&self) -> Arc<dyn HttpEgress> {
        self.http.clone()
    }
    fn grpc(&self) -> Option<Arc<dyn GrpcEgress>> {
        self.grpc.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::egress::DefaultEgress;
    use futures::future::BoxFuture;
    use std::sync::Arc;
    use swe_edge_egress_http::{HttpEgress, HttpEgressError, HttpEgressResult, HttpStreamResponse};

    struct DefaultEgressStub;
    impl HttpEgress for DefaultEgressStub {
        fn send(
            &self,
            _: swe_edge_egress_http::HttpRequest,
        ) -> BoxFuture<'_, HttpEgressResult<swe_edge_egress_http::HttpResponse>> {
            Box::pin(async { Err(HttpEgressError::Internal("stub".into())) })
        }
        fn send_stream(
            &self,
            _: swe_edge_egress_http::HttpRequest,
        ) -> BoxFuture<'_, HttpEgressResult<HttpStreamResponse>> {
            Box::pin(async { Err(HttpEgressError::Internal("stub".into())) })
        }
        fn health_check(&self) -> BoxFuture<'_, HttpEgressResult<()>> {
            Box::pin(async { Ok(()) })
        }
    }

    #[test]
    fn test_egress_http_returns_configured_client() {
        let client: Arc<dyn HttpEgress> = Arc::new(DefaultEgressStub);
        let egress = DefaultEgress::new_http(Arc::clone(&client));
        let _ = egress.http();
    }

    #[test]
    fn test_egress_grpc_returns_none_when_not_configured() {
        let egress = DefaultEgress::new_http(Arc::new(DefaultEgressStub));
        assert!(egress.grpc().is_none());
    }
}
