# Changelog

## [0.1.0] - 2026-06-20

### Added
- `CliRunner` trait — executes a parsed `CliCommand`
- `CliCommand` trait — parsed, dispatchable command representation
- `Validator` trait — validates CLI arguments before dispatch
- Value types: `CliArgs`, `CliOutput`, `CliError`
- `NoopCliRunner::create()` — pass-through runner for tests
- `NoopValidator::create()` — always-Ok validator for tests
