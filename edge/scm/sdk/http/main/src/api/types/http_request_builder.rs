//! Fluent builder for [`HttpRequest`].

use std::collections::HashMap;
use std::time::Duration;

use crate::api::types::http_body::HttpBody;
use crate::api::types::http_method::HttpMethod;
use crate::api::types::http_request::HttpRequest;

/// Fluent builder that constructs an [`HttpRequest`].
pub struct HttpRequestBuilder {
    method: HttpMethod,
    url: String,
    headers: HashMap<String, String>,
    query: HashMap<String, String>,
    body: Option<HttpBody>,
    timeout: Option<Duration>,
}

impl HttpRequestBuilder {
    /// Start a GET request to `url`.
    pub fn get(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Get,
            url: url.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            timeout: None,
        }
    }

    /// Start a POST request to `url`.
    pub fn post(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Post,
            url: url.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            timeout: None,
        }
    }

    /// Add a request header.
    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    /// Add a query parameter.
    pub fn with_query(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.insert(name.into(), value.into());
        self
    }

    /// Set the request body.
    pub fn with_body(mut self, body: HttpBody) -> Self {
        self.body = Some(body);
        self
    }

    /// Set the per-request timeout.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Consume the builder and return an [`HttpRequest`].
    pub fn build(self) -> HttpRequest {
        HttpRequest {
            method: self.method,
            url: self.url,
            headers: self.headers,
            query: self.query,
            body: self.body,
            timeout: self.timeout,
        }
    }
}
