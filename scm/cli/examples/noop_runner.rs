//! Demonstrates wiring a [`NoopCliRunner`] as the CLI surface.
#![allow(clippy::expect_used)]

use futures::executor::block_on;
use swe_edge_runtime_cli::{CliArgs, CliRunner, NoopCliRunner};

fn main() {
    let runner = NoopCliRunner::create();
    match block_on(runner.run("list", &CliArgs::new())) {
        Ok(out) => println!("exit_code={} stdout={:?}", out.exit_code, out.stdout),
        Err(e) => eprintln!("run failed: {e}"),
    }
}
