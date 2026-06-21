//! Demonstrates wiring a [`NoopCliRunner`] with a [`NoopCliCommand`].
#![allow(clippy::expect_used)]

use futures::executor::block_on;
use swe_edge_runtime_cli::{CliRunner, NoopCliCommand, NoopCliRunner};

fn main() {
    let runner = NoopCliRunner::create();
    let cmd = NoopCliCommand::create("list");
    match block_on(runner.run(&cmd)) {
        Ok(out) => println!("exit_code={} stdout={:?}", out.exit_code, out.stdout),
        Err(e) => eprintln!("run failed: {e}"),
    }
}
