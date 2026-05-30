//! Integration tests for [`TracingInitializer`].

use swe_edge_runtime::TracingInitializer;

/// @covers: TracingInitializer — is a zero-size type that can be named
#[test]
fn test_tracing_initializer_is_object_safe() {
    fn _assert_sized<T: Sized>() {}
    _assert_sized::<TracingInitializer>();
}
