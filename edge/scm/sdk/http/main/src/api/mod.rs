//! Public HTTP contract surface.

mod error;
mod noop;
mod traits;
mod types;

pub use error::HttpIngressError;
pub use noop::{NoopHttpIngress, NoopValidator};
pub use traits::{HttpIngress, Validator};
pub use types::{
    FormPart, HttpAuth, HttpBody, HttpDecodeFn, HttpEncodeFn, HttpHealthCheck, HttpIngressResult,
    HttpMethod, HttpRequest, HttpRequestBuilder, HttpResponse,
};
