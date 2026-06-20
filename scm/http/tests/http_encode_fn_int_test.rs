//! Tests for `HttpEncodeFn` — encode function type alias.
// @covers HttpEncodeFn
use swe_edge_runtime_http::{HttpEncodeFn, HttpResponse};

fn encode_string(body: String) -> HttpResponse {
    HttpResponse::new(200, body.into_bytes())
}

fn encode_error(_: ()) -> HttpResponse {
    HttpResponse::new(500, b"internal error".to_vec())
}

#[test]
fn test_http_encode_fn_success_happy() {
    let encode: HttpEncodeFn<String> = encode_string;
    let resp = encode("hello".to_string());
    assert_eq!(resp.status, 200);
    assert_eq!(resp.body, b"hello");
}

#[test]
fn test_http_encode_fn_error_response_error() {
    let encode: HttpEncodeFn<()> = encode_error;
    let resp = encode(());
    assert_eq!(resp.status, 500);
    assert!(resp.is_server_error());
}

#[test]
fn test_http_encode_fn_empty_body_edge() {
    fn encode_empty(_: u8) -> HttpResponse {
        HttpResponse::new(204, vec![])
    }
    let encode: HttpEncodeFn<u8> = encode_empty;
    let resp = encode(0);
    assert_eq!(resp.status, 204);
    assert!(resp.body.is_empty());
}
