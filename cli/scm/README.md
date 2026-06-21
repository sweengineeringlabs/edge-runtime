# edge-runtime-cli

CLI runner/command contracts for the swe-edge runtime layer.

**Contracts only** — no clap, no argh, no structopt dependencies.

## Usage

```rust
use swe_edge_runtime_cli::{CliRunner, CliCommand, CliArgs, CliOutput, NoopCliRunner};

// Use the Noop runner in tests
let runner = NoopCliRunner::create();
```

## Traits

| Trait | Purpose |
|-------|---------|
| `CliRunner` | Executes a parsed `CliCommand` |
| `CliCommand` | Parsed, dispatchable command |
| `Validator` | Validates CLI arguments before dispatch |
