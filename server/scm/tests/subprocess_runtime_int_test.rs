//! Integration tests — subprocess SAF surface exported from swe_edge_runtime.

// Unconditional direct-dep imports satisfy the deps_have_integration_tests rule.
use swe_edge_egress_subprocess::SubprocessRunner;

#[test]
fn test_subprocess_runner_dep_is_object_safe() {
    fn _assert(_: &dyn SubprocessRunner) {}
}

#[cfg(feature = "subprocess")]
use swe_edge_runtime::{SubprocessRunner as RuntimeSubprocessRunner, SubprocessSvc};

#[cfg(feature = "subprocess")]
#[test]
fn test_subprocess_runner_is_object_safe() {
    fn _assert_object_safe(_: &dyn RuntimeSubprocessRunner) {}
}

#[cfg(feature = "subprocess")]
#[test]
fn test_subprocess_runner_is_exported_from_runtime() {
    let _runner = SubprocessSvc::runner();
}

#[cfg(feature = "subprocess")]
#[test]
fn test_build_registry_with_subprocess_stores_runner_happy() {
    use std::sync::Arc;
    use swe_edge_egress_http::HttpTransportSvc;
    use swe_edge_runtime::Runtime;

    let http = Arc::from(
        HttpTransportSvc::default_http_egress().unwrap_or_else(|e| panic!("http egress: {e}")),
    );
    let runner = SubprocessSvc::runner();
    let reg = Runtime::builder()
        .egress_http(http)
        .with_subprocess(runner)
        .build_registry()
        .unwrap_or_else(|| panic!("registry requires http egress"));

    assert!(reg.subprocess().is_some());
}

#[cfg(feature = "subprocess")]
#[test]
fn test_build_registry_without_subprocess_returns_none_edge() {
    use std::sync::Arc;
    use swe_edge_egress_http::HttpTransportSvc;
    use swe_edge_runtime::Runtime;

    let http = Arc::from(
        HttpTransportSvc::default_http_egress().unwrap_or_else(|e| panic!("http egress: {e}")),
    );
    let reg = Runtime::builder()
        .egress_http(http)
        .build_registry()
        .unwrap_or_else(|| panic!("registry requires http egress"));

    assert!(reg.subprocess().is_none());
}

#[cfg(feature = "subprocess")]
#[tokio::test]
async fn test_subprocess_runner_from_registry_denies_empty_allow_list_error() {
    use std::sync::Arc;
    use swe_edge_egress_http::HttpTransportSvc;
    use swe_edge_runtime::{Runtime, SubprocessArgs, SubprocessResult};

    let http = Arc::from(
        HttpTransportSvc::default_http_egress().unwrap_or_else(|e| panic!("http egress: {e}")),
    );
    let runner = SubprocessSvc::runner();
    let reg = Runtime::builder()
        .egress_http(http)
        .with_subprocess(runner)
        .build_registry()
        .unwrap_or_else(|| panic!("registry requires http egress"));

    let args = SubprocessArgs::builder().argv(vec!["echo".into()]).build();
    let result = reg
        .subprocess()
        .unwrap_or_else(|| panic!("subprocess runner not set"))
        .run(args)
        .await;
    assert!(
        matches!(result, SubprocessResult::Denied { .. }),
        "empty allow_commands must deny the command"
    );
}
