//! [`Input`] trait impl for [`DefaultInput`].

use std::sync::Arc;

use swe_edge_ingress::{GrpcInbound, HttpInbound};

use crate::api::input::{DefaultInput, Input};

impl Input for DefaultInput {
    fn http(&self) -> Option<Arc<dyn HttpInbound>> { self.http.clone() }
    fn grpc(&self) -> Option<Arc<dyn GrpcInbound>> { self.grpc.clone() }
}
