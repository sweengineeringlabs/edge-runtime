//! No-op [`HttpIngress`] implementation.

use edge_domain::SecurityContext;
use futures::future::BoxFuture;

use crate::api::{
    HttpHealthCheck, HttpIngress, HttpIngressResult, HttpRequest, HttpResponse, NoopHttpIngress,
};

impl HttpIngress for NoopHttpIngress {
    fn handle(
        &self,
        _req: &HttpRequest,
        _ctx: SecurityContext,
    ) -> BoxFuture<'_, HttpIngressResult<HttpResponse>> {
        Box::pin(async { Ok(HttpResponse::new(200, vec![])) })
    }

    fn health_check(&self) -> BoxFuture<'_, HttpIngressResult<HttpHealthCheck>> {
        Box::pin(async { Ok(HttpHealthCheck::healthy()) })
    }
}
