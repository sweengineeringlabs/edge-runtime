//! Integration tests — subprocess SAF surface exported from swe_edge_runtime.

#[cfg(feature = "subprocess")]
use swe_edge_runtime::{SubprocessRunner, SubprocessSvc};

#[cfg(feature = "subprocess")]
#[test]
fn test_subprocess_runner_is_object_safe() {
    fn _assert_object_safe(_: &dyn SubprocessRunner) {}
}

#[cfg(feature = "subprocess")]
#[test]
fn test_subprocess_runner_is_exported_from_runtime() {
    let _runner = SubprocessSvc::runner();
}

#[cfg(feature = "subprocess")]
#[test]
fn test_build_registry_with_subprocess_stores_runner() {
    use std::sync::Arc;
    use swe_edge_egress_http::HttpTransportSvc;
    use swe_edge_runtime::Runtime;

    let http = Arc::from(HttpTransportSvc::default_http_egress().expect("http egress"));
    let runner = SubprocessSvc::runner();
    let reg = Runtime::builder()
        .egress_http(http)
        .with_subprocess(runner)
        .build_registry()
        .expect("registry requires http egress");

    assert!(reg.subprocess().is_some());
}

#[cfg(feature = "subprocess")]
#[test]
fn test_build_registry_without_subprocess_returns_none() {
    use std::sync::Arc;
    use swe_edge_egress_http::HttpTransportSvc;
    use swe_edge_runtime::Runtime;

    let http = Arc::from(HttpTransportSvc::default_http_egress().expect("http egress"));
    let reg = Runtime::builder()
        .egress_http(http)
        .build_registry()
        .expect("registry");

    assert!(reg.subprocess().is_none());
}

#[cfg(feature = "subprocess")]
#[tokio::test]
async fn test_subprocess_runner_from_registry_denies_empty_allow_list() {
    use std::sync::Arc;
    use swe_edge_egress_http::HttpTransportSvc;
    use swe_edge_runtime::{Runtime, SubprocessArgs, SubprocessResult};

    let http = Arc::from(HttpTransportSvc::default_http_egress().expect("http egress"));
    let runner = SubprocessSvc::runner();
    let reg = Runtime::builder()
        .egress_http(http)
        .with_subprocess(runner)
        .build_registry()
        .expect("registry");

    let args = SubprocessArgs::builder().argv(vec!["echo".into()]).build();
    let result = reg.subprocess().expect("runner").run(args).await;
    assert!(
        matches!(result, SubprocessResult::Denied { .. }),
        "empty allow_commands must deny the command"
    );
}
