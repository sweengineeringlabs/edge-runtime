//! HTTP request type.

use std::collections::HashMap;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::api::types::http_body::HttpBody;
use crate::api::types::http_method::HttpMethod;

/// An inbound or outbound HTTP request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpRequest {
    /// HTTP method.
    pub method: HttpMethod,
    /// Request URL.
    pub url: String,
    /// Request headers.
    #[serde(default)]
    pub headers: HashMap<String, String>,
    /// Query string parameters.
    #[serde(default)]
    pub query: HashMap<String, String>,
    /// Optional request body.
    pub body: Option<HttpBody>,
    /// Per-request timeout override.
    pub timeout: Option<Duration>,
}

impl HttpRequest {
    /// Construct a GET request.
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

    /// Construct a POST request.
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

    /// Construct a PUT request.
    pub fn put(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Put,
            url: url.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            timeout: None,
        }
    }

    /// Construct a DELETE request.
    pub fn delete(url: impl Into<String>) -> Self {
        Self {
            method: HttpMethod::Delete,
            url: url.into(),
            headers: HashMap::new(),
            query: HashMap::new(),
            body: None,
            timeout: None,
        }
    }

    /// Look up a header value (RFC 7230 case-insensitive).
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .get(name)
            .or_else(|| self.headers.get(&name.to_lowercase()))
            .map(String::as_str)
            .or_else(|| {
                self.headers
                    .iter()
                    .find(|(k, _)| k.eq_ignore_ascii_case(name))
                    .map(|(_, v)| v.as_str())
            })
    }

    /// Add a header.
    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    /// Add a query parameter.
    pub fn with_query(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.insert(name.into(), value.into());
        self
    }

    /// Set a JSON body.
    pub fn with_json<T: serde::Serialize>(mut self, body: &T) -> Result<Self, serde_json::Error> {
        self.body = Some(HttpBody::Json(serde_json::to_value(body)?));
        self.headers
            .insert("Content-Type".to_string(), "application/json".to_string());
        Ok(self)
    }

    /// Set a raw bytes body with the given content type.
    pub fn with_body(mut self, body: Vec<u8>, content_type: impl Into<String>) -> Self {
        self.body = Some(HttpBody::Raw(body));
        self.headers
            .insert("Content-Type".to_string(), content_type.into());
        self
    }

    /// Set a URL-encoded form body.
    pub fn with_form(mut self, form: HashMap<String, String>) -> Self {
        self.body = Some(HttpBody::Form(form));
        self.headers.insert(
            "Content-Type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );
        self
    }

    /// Set a per-request timeout override.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
}
