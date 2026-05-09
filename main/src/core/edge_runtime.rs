//! `EdgeRuntimeBuilder::serve()` implementation.

use std::sync::Arc;

use edge_proxy::new_null_lifecycle_monitor;

use crate::api::edge_runtime::EdgeRuntimeBuilder;
use crate::api::error::{RuntimeError, RuntimeResult};
use crate::api::input::{DefaultInput, Input};
use crate::api::output::DefaultOutput;
use crate::saf::{load_config, run};

impl EdgeRuntimeBuilder {
    /// Assemble all registered components and start the runtime.
    ///
    /// Priority for inbound:
    /// - Routes registered via `http_route` / `grpc_route` (builds dispatcher internally).
    /// - Fall back to a pre-built handler supplied via `http_handler` / `grpc_handler`.
    ///
    /// Returns `Err` when neither routes nor handlers were registered for any transport.
    pub async fn serve(self) -> RuntimeResult<()> {
        let config = match self.config {
            Some(c) => c,
            None    => load_config().map_err(|e| RuntimeError::StartFailed(e.to_string()))?,
        };

        let mut input = DefaultInput::empty();

        // HTTP: internal dispatcher takes precedence over pre-built handler.
        if let Some(d) = self.http_dispatcher {
            input = input.with_http(Arc::new(d));
        } else if let Some(h) = self.http_handler {
            input = input.with_http(h);
        }

        // gRPC: same priority rule.
        if let Some(d) = self.grpc_dispatcher {
            input = input.with_grpc(Arc::new(d));
        } else if let Some(h) = self.grpc_handler {
            input = input.with_grpc(h);
        }

        if !input.has_any() {
            return Err(RuntimeError::StartFailed(
                "EdgeRuntime: no handler registered — call .http_route() or .grpc_route()".into(),
            ));
        }

        let egress_http = self.egress_http.ok_or_else(|| {
            RuntimeError::StartFailed(
                "EdgeRuntime: no HTTP egress client registered — call .egress_http()".into(),
            )
        })?;
        let mut output = DefaultOutput::new_http(egress_http);
        if let Some(g) = self.egress_grpc {
            output = output.with_grpc(g);
        }

        let lifecycle = self.lifecycle.unwrap_or_else(|| new_null_lifecycle_monitor());

        run(config, Arc::new(input), Arc::new(output), lifecycle).await
    }
}
