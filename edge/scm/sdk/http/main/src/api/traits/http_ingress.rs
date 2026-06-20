//! HTTP inbound contract — receives and handles inbound HTTP requests.

use edge_domain::SecurityContext;
use futures::future::BoxFuture;

use crate::api::{
    FormPart, HttpAuth, HttpBody, HttpDecodeFn, HttpEncodeFn, HttpHealthCheck, HttpIngressError,
    HttpIngressResult, HttpMethod, HttpRequest, HttpRequestBuilder, HttpResponse,
};

/// Receives and handles inbound HTTP requests.
///
/// Implement this trait in a plugin or transport binding. The composition root
/// wires implementors into the HTTP transport crate which drives the server loop.
pub trait HttpIngress: Send + Sync {
    /// Handle an inbound HTTP request and return a response.
    fn handle(
        &self,
        req: &HttpRequest,
        ctx: SecurityContext,
    ) -> BoxFuture<'_, HttpIngressResult<HttpResponse>>;

    /// Perform a health check of this ingress handler.
    fn health_check(&self) -> BoxFuture<'_, HttpIngressResult<HttpHealthCheck>>;

    /// Return the HTTP methods accepted by this handler.
    fn accepted_methods(&self) -> Vec<HttpMethod> {
        vec![]
    }

    /// Create a default request builder pointed at the root path.
    fn request_builder(&self) -> HttpRequestBuilder {
        HttpRequestBuilder::get("/")
    }

    /// Extract the auth credential from a request, if present.
    fn extract_auth(&self, _req: &HttpRequest) -> Option<HttpAuth> {
        None
    }

    /// Extract the request body, if present.
    fn extract_body<'r>(&self, req: &'r HttpRequest) -> Option<&'r HttpBody> {
        req.body.as_ref()
    }

    /// Extract multipart form parts from a request.
    fn extract_form_parts(&self, _req: &HttpRequest) -> Vec<FormPart> {
        vec![]
    }

    /// Wrap a request decode function (used by adapters).
    fn wrap_decode_fn<Req>(&self, decode: HttpDecodeFn<Req>) -> HttpDecodeFn<Req>
    where
        Req: 'static,
        Self: Sized,
    {
        decode
    }

    /// Wrap a response encode function (used by adapters).
    fn wrap_encode_fn<Resp>(&self, encode: HttpEncodeFn<Resp>) -> HttpEncodeFn<Resp>
    where
        Resp: 'static,
        Self: Sized,
    {
        encode
    }

    /// Return the error kind label for a given ingress error.
    fn error_kind(&self, _err: &HttpIngressError) -> &'static str {
        "ingress_error"
    }
}
