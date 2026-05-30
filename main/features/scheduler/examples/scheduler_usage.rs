//! Example: create a tokio-backed scheduler and drive an async future.

use swe_edge_runtime_scheduler::{Scheduler, SchedulerSvc, TokioSchedulerConfig};

fn main() {
    let config = TokioSchedulerConfig {
        thread_name: Some("example-worker".into()),
        ..Default::default()
    };
    let scheduler = SchedulerSvc::tokio_scheduler(config, "example");

    let result: Result<(), _> = scheduler.run(async {
        println!("Running inside the tokio scheduler!");
    });

    if let Err(e) = result {
        eprintln!("scheduler failed: {e}");
        std::process::exit(1);
    }
}
