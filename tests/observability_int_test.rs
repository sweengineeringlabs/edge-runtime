//! Integration tests for `init_tracing` and `TracingFormat`.

#[cfg(feature = "observability")]
use swe_edge_runtime::{TracingFormat, init_tracing};

/// @covers: init_tracing — Json format installs without panic
#[cfg(feature = "observability")]
#[test]
fn test_observability_struct_init_tracing_json_installs_without_panic() {
    init_tracing(TracingFormat::Json);
}

/// @covers: init_tracing — Pretty format installs without panic
#[cfg(feature = "observability")]
#[test]
fn test_observability_struct_init_tracing_pretty_installs_without_panic() {
    init_tracing(TracingFormat::Pretty);
}

/// @covers: init_tracing — idempotent across multiple calls
#[cfg(feature = "observability")]
#[test]
fn test_observability_struct_init_tracing_called_twice_is_idempotent() {
    init_tracing(TracingFormat::Json);
    init_tracing(TracingFormat::Pretty);
}
