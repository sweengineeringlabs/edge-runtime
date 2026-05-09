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
    /// Blocks until a shutdown signal (SIGTERM / SIGINT) is received or an
    /// error occurs.  Returns `Err(RuntimeError::NoTransportConfigured)` when
    /// neither `http_handler` nor `grpc_handler` was registered.
    pub async fn serve(self) -> RuntimeResult<()> {
        let config = match self.config {
            Some(c) => c,
            None    => load_config().map_err(|e| RuntimeError::StartFailed(e.to_string()))?,
        };

        let mut input = DefaultInput::empty();
        if let Some(h) = self.http_handler {
            input = input.with_http(h);
        }
        if let Some(h) = self.grpc_handler {
            input = input.with_grpc(h);
        }
        if !input.has_any() {
            return Err(RuntimeError::StartFailed(
                "EdgeRuntimeBuilder: no ingress handler registered (call .http_handler() or .grpc_handler())".into()
            ));
        }

        let egress_http = self.egress_http.ok_or_else(|| {
            RuntimeError::StartFailed(
                "EdgeRuntimeBuilder: no HTTP egress client registered (call .egress_http())".into()
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
