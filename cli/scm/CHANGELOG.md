# Changelog

## [0.1.1] - 2026-06-20

### Added
- `CliCommand` trait — parsed, dispatchable command representation (`name()`, `args()`)
- `NoopCliCommand::create(name)` / `create_with_args(name, args)` — test stub
- `CliRunner::run` now accepts `&dyn CliCommand` instead of `(name: &str, args: &CliArgs)`
- `saf/cli/` submodule groups `cli_command_svc` and `cli_runner_svc`

## [0.1.0] - 2026-06-20

### Added
- `CliRunner` trait — executes a parsed `CliCommand`
- `Validator` trait — validates CLI arguments before dispatch
- Value types: `CliArgs`, `CliOutput`, `CliError`
- `NoopCliRunner::create()` — pass-through runner for tests
- `NoopValidator::create()` — always-Ok validator for tests
