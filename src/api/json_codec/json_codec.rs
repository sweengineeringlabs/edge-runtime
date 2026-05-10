//! JSON codec type aliases — default HTTP/gRPC decode/encode functions.
//!
//! These are the function-pointer types used by `http_route` and `grpc_route`
//! when no explicit codec is supplied by the caller.

/// Marker trait for types that can be used as JSON codecs.
pub trait JsonCodec: Send + Sync {}

use swe_edge_ingress::{HttpInboundError, HttpRequest, HttpResponse, GrpcInboundError};

/// Default JSON decode: deserialises the HTTP request body into `Req`.
pub(crate) type JsonHttpDecodeFn<Req> = fn(&HttpRequest) -> Result<Req, HttpInboundError>;

/// Default JSON encode: serialises `Resp` into a `200 application/json` response.
pub(crate) type JsonHttpEncodeFn<Resp> = fn(Resp) -> HttpResponse;

/// Default gRPC JSON decode: deserialises raw bytes into `Req`.
pub(crate) type JsonGrpcDecodeFn<Req> = fn(&[u8]) -> Result<Req, GrpcInboundError>;

/// Default gRPC JSON encode: serialises `Resp` to raw bytes.
pub(crate) type JsonGrpcEncodeFn<Resp> = fn(&Resp) -> Vec<u8>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_http_decode_fn_type_alias_is_well_formed() {
        use swe_edge_ingress::HttpRequest;
        fn _accepts<Req>(_f: JsonHttpDecodeFn<Req>) {}
        fn sample_decode(req: &HttpRequest) -> Result<String, HttpInboundError> {
            req.body.as_ref().map(|_| "ok".to_string())
                .ok_or_else(|| HttpInboundError::InvalidInput("empty".into()))
        }
        _accepts(sample_decode);
    }

    #[test]
    fn test_json_grpc_decode_fn_type_alias_is_well_formed() {
        fn _accepts<Req>(_f: JsonGrpcDecodeFn<Req>) {}
        fn sample_decode(b: &[u8]) -> Result<String, GrpcInboundError> {
            std::str::from_utf8(b).map(|s| s.to_string())
                .map_err(|e| GrpcInboundError::InvalidArgument(e.to_string()))
        }
        _accepts(sample_decode);
    }
}
