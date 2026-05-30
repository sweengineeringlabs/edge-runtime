//! Integration tests for the IsolationProfileRegistry public API.

use std::sync::Arc;

use swe_edge_egress_subprocess::{
    IsolationError, IsolationProfile, SubprocessArgs, SubprocessResult, SubprocessRunner,
    SubprocessSvc,
};
use swe_edge_runtime_isolator::{IsolatorConfig, IsolatorSvc};

// ── NoopIsolator ──────────────────────────────────────────────────────────────

/// @covers: IsolatorSvc::create_noop_isolator — name is "noop".
#[test]
fn test_create_noop_isolator_name_is_noop() {
    assert_eq!(IsolatorSvc::create_noop_isolator().name(), "noop");
}

/// @covers: IsolatorSvc::create_noop_isolator — can be stored as Arc<dyn IsolationProfile>.
#[test]
fn test_create_noop_isolator_storable_as_arc_dyn() {
    let _: Arc<dyn IsolationProfile> = Arc::new(IsolatorSvc::create_noop_isolator());
}

/// @covers: IsolatorSvc::create_noop_isolator — configure returns Ok.
#[test]
fn test_noop_isolator_configure_returns_ok() {
    let mut cmd = tokio::process::Command::new("echo");
    assert!(IsolatorSvc::create_noop_isolator()
        .configure(&mut cmd)
        .is_ok());
}

// ── IsolationProfileRegistry ─────────────────────────────────────────────────

/// @covers: IsolationProfileRegistry — "noop" built-in always present.
#[test]
fn test_registry_noop_profile_always_present() {
    let registry = IsolatorSvc::build_registry(IsolatorConfig::default()).expect("registry build");
    assert!(registry.get("noop").is_ok());
}

/// @covers: IsolationProfileRegistry — "default" maps to noop.
#[test]
fn test_registry_default_profile_resolves_to_noop() {
    let registry = IsolatorSvc::build_registry(IsolatorConfig::default()).expect("registry build");
    let profile = registry.get("default").unwrap();
    assert_eq!(profile.name(), "noop");
}

/// @covers: IsolationProfileRegistry::get — unknown name returns UnknownProfile.
#[test]
fn test_registry_get_unknown_returns_unknown_profile_error() {
    let registry = IsolatorSvc::build_registry(IsolatorConfig::default()).expect("registry build");
    let err = registry.get("nonexistent").unwrap_err();
    assert!(
        matches!(err, IsolationError::UnknownProfile { .. }),
        "expected UnknownProfile; got {err:?}",
    );
}

// ── SubprocessRunner integration ─────────────────────────────────────────────

/// @covers: SubprocessRunner — NoopIsolator does not affect a successful run.
#[tokio::test]
async fn test_subprocess_runner_with_noop_isolator_completes() {
    use swe_edge_egress_subprocess::SubprocessRunner as _;

    let profile: Arc<dyn IsolationProfile> = Arc::new(IsolatorSvc::create_noop_isolator());

    #[cfg(unix)]
    let (argv, allow) = (
        vec!["echo".to_owned(), "hi".to_owned()],
        vec!["echo".to_owned()],
    );
    #[cfg(windows)]
    let (argv, allow) = (
        vec!["cmd".to_owned(), "/C".to_owned(), "echo hi".to_owned()],
        vec!["cmd".to_owned()],
    );

    let args = SubprocessArgs::builder()
        .argv(argv)
        .allow_commands(allow)
        .timeout_ms(5_000)
        .isolation_profile(profile)
        .build();

    let result = SubprocessSvc::runner().run(args).await;
    assert!(
        matches!(result, SubprocessResult::Completed { exit_code: 0, .. }),
        "expected Completed(0); got {result:?}",
    );
}

/// @covers: SubprocessRunner — failing apply hook returns IsolationFailed.
#[tokio::test]
async fn test_subprocess_runner_isolation_failed_when_apply_errors() {
    use swe_edge_egress_subprocess::SubprocessRunner as _;

    #[derive(Debug)]
    struct FailingIsolator;
    impl IsolationProfile for FailingIsolator {
        fn name(&self) -> &str {
            "failing"
        }
        fn apply(&self, _child: &mut tokio::process::Child) -> Result<(), IsolationError> {
            Err(IsolationError::UnsupportedPlatform {
                profile: "failing".into(),
            })
        }
    }

    let profile: Arc<dyn IsolationProfile> = Arc::new(FailingIsolator);

    #[cfg(unix)]
    let (argv, allow) = (
        vec!["echo".to_owned(), "hi".to_owned()],
        vec!["echo".to_owned()],
    );
    #[cfg(windows)]
    let (argv, allow) = (
        vec!["cmd".to_owned(), "/C".to_owned(), "echo hi".to_owned()],
        vec!["cmd".to_owned()],
    );

    let args = SubprocessArgs::builder()
        .argv(argv)
        .allow_commands(allow)
        .timeout_ms(5_000)
        .isolation_profile(profile)
        .build();

    let result = SubprocessSvc::runner().run(args).await;
    assert!(
        matches!(result, SubprocessResult::IsolationFailed { .. }),
        "expected IsolationFailed; got {result:?}",
    );
}
