//! Service Abstraction Framework — the only public export surface for consumers.

mod http_ingress_svc;
mod server_svc;
mod validator_svc;

pub use http_ingress_svc::{
    FormPart, HttpAuth, HttpBody, HttpDecodeFn, HttpEncodeFn, HttpHealthCheck, HttpIngress,
    HttpIngressError, HttpIngressResult, HttpMethod, HttpRequest, HttpRequestBuilder, HttpResponse,
    NoopHttpIngress,
};
pub use server_svc::{
    AxumHttpServer, AxumHttpServerBuilder, AxumHttpServerHelper, HttpServer, HttpServerError,
    HTTP_SERVER_SVC,
};
pub use validator_svc::{NoopValidator, Validator};
