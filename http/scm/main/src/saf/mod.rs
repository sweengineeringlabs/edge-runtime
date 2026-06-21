//! Service Abstraction Framework — the only public export surface for consumers.

mod http;
mod server_svc;
mod validator_svc;

pub use http::{
    FormPart, HttpAuth, HttpBody, HttpDecodeFn, HttpEncodeFn, HttpHealthCheck, HttpIngress,
    HttpIngressError, HttpIngressResult, HttpMethod, HttpRequest, HttpRequestBuilder, HttpResponse,
    HttpServerSvc, NoopHttpIngress,
};
pub use server_svc::{
    AxumHttpServer, AxumHttpServerBuilder, AxumHttpServerHelper, HttpServer, HttpServerError,
    HTTP_SERVER_SVC,
};
pub use validator_svc::{NoopValidator, Validator};
