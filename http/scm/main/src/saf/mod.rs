//! Service Abstraction Framework — trait re-exports and facades.

mod http;
mod server_svc;
mod validator_svc;

pub use http::HttpIngress;
pub use server_svc::HTTP_SERVER_SVC;
pub use swe_edge_ingress_http::{
    FormPart, HttpAuth, HttpBody, HttpDecodeFn, HttpEncodeFn, HttpHealthCheck, HttpIngressError,
    HttpIngressResult, HttpMethod, HttpRequest, HttpRequestBuilder, HttpResponse,
};
pub use validator_svc::Validator;
