//! [`Input`] trait impl for [`DefaultInput`].

use std::sync::Arc;

use swe_edge_ingress::{GrpcInbound, HttpInbound};

use crate::api::input::{DefaultInput, Input};

impl Input for DefaultInput {
    fn http(&self) -> Option<Arc<dyn HttpInbound>> { self.http.clone() }
    fn grpc(&self) -> Option<Arc<dyn GrpcInbound>> { self.grpc.clone() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::input::DefaultInput;

    /// @covers: http
    #[test]
    fn test_input_http_returns_none_when_not_set() {
        let input = DefaultInput::empty();
        assert!(input.http().is_none());
    }

    /// @covers: grpc
    #[test]
    fn test_input_grpc_returns_none_when_not_set() {
        let input = DefaultInput::empty();
        assert!(input.grpc().is_none());
    }
}
