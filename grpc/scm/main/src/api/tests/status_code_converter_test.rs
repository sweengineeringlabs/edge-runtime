//! Contract tests for [`StatusCodeConverter`] public functions.

#[cfg(test)]
mod tests {
    use crate::api::{StatusCodeConvert, StatusCodeConverter};
    use swe_edge_ingress_grpc::GrpcStatusCode;

    /// @covers: to_wire
    #[test]
    fn test_to_wire_ok_is_zero_happy() {
        assert_eq!(StatusCodeConverter::to_wire(GrpcStatusCode::Ok), 0);
    }

    /// @covers: from_wire
    #[test]
    fn test_from_wire_zero_is_ok_happy() {
        assert_eq!(StatusCodeConverter::from_wire(0), GrpcStatusCode::Ok);
    }

    /// @covers: from_wire
    #[test]
    fn test_from_wire_unknown_code_maps_to_unknown_edge() {
        assert_eq!(StatusCodeConverter::from_wire(2), GrpcStatusCode::Unknown);
    }

    /// @covers: to_wire
    #[test]
    fn test_to_wire_unknown_maps_to_nonzero_error() {
        assert_ne!(StatusCodeConverter::to_wire(GrpcStatusCode::Unknown), 0);
    }
}
