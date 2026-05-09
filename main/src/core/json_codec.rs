//! Default JSON codec for HTTP and gRPC routes.
//!
//! `http_route` and `grpc_route` use these automatically when `Req` and `Resp`
//! implement `serde::de::DeserializeOwned` / `serde::Serialize`.

use swe_edge_ingress::{
    GrpcInboundError, HttpBody, HttpInboundError, HttpRequest, HttpResponse,
};

/// Default HTTP decode: JSON body → `Req`.
///
/// Handles `application/json` (parsed `HttpBody::Json`) and raw bytes
/// (`HttpBody::Raw`) transparently.  Returns `InvalidInput` on any other body
/// type or a deserialization failure.
pub(crate) fn json_decode<Req: serde::de::DeserializeOwned>(
    req: &HttpRequest,
) -> Result<Req, HttpInboundError> {
    match &req.body {
        Some(HttpBody::Json(v)) => serde_json::from_value(v.clone())
            .map_err(|e| HttpInboundError::InvalidInput(e.to_string())),
        Some(HttpBody::Raw(b)) => serde_json::from_slice(b)
            .map_err(|e| HttpInboundError::InvalidInput(e.to_string())),
        None => serde_json::from_slice(b"null")
            .map_err(|e| HttpInboundError::InvalidInput(e.to_string())),
        Some(_) => Err(HttpInboundError::InvalidInput(
            "expected JSON body (application/json or raw bytes)".into(),
        )),
    }
}

/// Default HTTP encode: `Resp` → 200 JSON response.
pub(crate) fn json_encode<Resp: serde::Serialize>(resp: Resp) -> HttpResponse {
    match serde_json::to_vec(&resp) {
        Ok(body) => {
            let mut r = HttpResponse::new(200, body);
            r.headers.insert("content-type".into(), "application/json".into());
            r
        }
        Err(e) => HttpResponse::new(500, e.to_string().into_bytes()),
    }
}

/// Default gRPC decode: raw bytes → `Req` via JSON.
pub(crate) fn grpc_json_decode<Req: serde::de::DeserializeOwned>(
    bytes: &[u8],
) -> Result<Req, GrpcInboundError> {
    serde_json::from_slice(bytes)
        .map_err(|e| GrpcInboundError::InvalidArgument(e.to_string()))
}

/// Default gRPC encode: `Resp` → raw bytes via JSON.
pub(crate) fn grpc_json_encode<Resp: serde::Serialize>(resp: &Resp) -> Vec<u8> {
    serde_json::to_vec(resp).unwrap_or_default()
}
