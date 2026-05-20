//! Example: bring your own scheduler implementation.
//!
//! Run with: `cargo run --example custom_scheduler`

use std::future::Future;

use swe_edge_runtime::{Runtime, RuntimeResult};
use swe_edge_runtime_scheduler::{run_with_scheduler, Scheduler};

/// A scheduler backed by a single-threaded tokio runtime.
struct SingleThreadScheduler;

impl Scheduler for SingleThreadScheduler {
    fn run<F>(&self, fut: F) -> RuntimeResult<()>
    where
        F: Future<Output = RuntimeResult<()>> + Send + 'static,
    {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("build single-thread runtime")
            .block_on(fut)
    }
}

fn main() {
    let result = run_with_scheduler(
        Runtime::builder().app_name("custom-scheduler-example"),
        SingleThreadScheduler,
    );
    // StartFailed expected — no handler registered. In real usage, register routes first.
    eprintln!("result: {result:?}");
}
