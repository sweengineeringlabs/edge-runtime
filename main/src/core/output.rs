//! [`Output`] trait impl for [`DefaultOutput`].

use std::sync::Arc;

use swe_edge_egress_grpc::GrpcOutbound;
use swe_edge_egress_http::HttpOutbound;

use crate::api::output::{DefaultOutput, Output};

impl Output for DefaultOutput {
    fn http(&self) -> Arc<dyn HttpOutbound>         { self.http.clone() }
    fn grpc(&self) -> Option<Arc<dyn GrpcOutbound>> { self.grpc.clone() }
}
