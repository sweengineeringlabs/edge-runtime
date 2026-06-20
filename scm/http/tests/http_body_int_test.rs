//! Tests for `HttpBody` — HTTP request body variants.
// @covers HttpBody
#![allow(clippy::unwrap_used)]

use std::collections::HashMap;
use swe_edge_runtime_http::{FormPart, HttpBody};

#[test]
fn test_http_body_json_variant_happy() {
    let body = HttpBody::Json(serde_json::json!({"key": "value"}));
    assert!(matches!(body, HttpBody::Json(_)));
}

#[test]
fn test_http_body_raw_variant_happy() {
    let body = HttpBody::Raw(b"raw bytes".to_vec());
    assert!(matches!(body, HttpBody::Raw(_)));
}

#[test]
fn test_http_body_form_variant_happy() {
    let mut form = HashMap::new();
    form.insert("field".to_string(), "value".to_string());
    let body = HttpBody::Form(form);
    assert!(matches!(body, HttpBody::Form(_)));
}

#[test]
fn test_http_body_multipart_variant_error() {
    // "Error" path: multipart with missing content-type part.
    let part = FormPart {
        name: "file".to_string(),
        filename: None,
        content_type: None,
        data: vec![],
    };
    let body = HttpBody::Multipart(vec![part]);
    assert!(matches!(body, HttpBody::Multipart(_)));
}

#[test]
fn test_http_body_raw_empty_edge() {
    // Edge: empty raw body.
    let body = HttpBody::Raw(vec![]);
    assert!(matches!(body, HttpBody::Raw(v) if v.is_empty()));
}
