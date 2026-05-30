//! [`TokioScheduler`] — tokio-backed implementation of [`Scheduler`].

use std::future::Future;
use std::sync::OnceLock;

use crate::api::error::SchedulerError;
use crate::api::scheduler::Scheduler;
use crate::api::types::TokioSchedulerConfig;

/// Drives the process runtime using a tokio multi-thread scheduler.
pub struct TokioScheduler {
    pub(crate) config: TokioSchedulerConfig,
    pub(crate) thread_name: String,
}

static PANIC_HOOK: OnceLock<()> = OnceLock::new();

impl TokioScheduler {
    fn install_panic_hook(&self) {
        PANIC_HOOK.get_or_init(|| {
            let prev = std::panic::take_hook();
            std::panic::set_hook(Box::new(move |info| {
                tracing::error!(panic = %info, "worker thread panicked — process will abort");
                prev(info);
            }));
        });
    }
}

impl Scheduler for TokioScheduler {
    fn run<F, T>(&self, fut: F) -> Result<T, SchedulerError>
    where
        F: Future<Output = T> + Send + 'static,
    {
        self.install_panic_hook();

        let mut builder = tokio::runtime::Builder::new_multi_thread();
        builder.enable_all();
        builder.thread_name(&self.thread_name);

        if let Some(workers) = self.config.workers {
            builder.worker_threads(workers.get());
        }
        if let Some(stack_kib) = self.config.thread_stack_kib {
            builder.thread_stack_size(stack_kib * 1024);
        }
        if let Some(max_blocking) = self.config.max_blocking_threads {
            builder.max_blocking_threads(max_blocking);
        }

        let rt = builder
            .build()
            .map_err(|e| SchedulerError::StartFailed(format!("tokio: {e}")))?;

        Ok(rt.block_on(fut))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_scheduler(config: TokioSchedulerConfig, thread_name: &str) -> TokioScheduler {
        let thread_name = config
            .thread_name
            .clone()
            .unwrap_or_else(|| thread_name.to_owned());
        TokioScheduler {
            config,
            thread_name,
        }
    }

    #[test]
    fn test_construction_uses_config_thread_name_when_set() {
        let cfg = TokioSchedulerConfig {
            thread_name: Some("cfg".into()),
            ..Default::default()
        };
        assert_eq!(make_scheduler(cfg, "arg").thread_name, "cfg");
    }

    #[test]
    fn test_construction_falls_back_to_arg_when_config_thread_name_absent() {
        assert_eq!(
            make_scheduler(TokioSchedulerConfig::default(), "arg").thread_name,
            "arg"
        );
    }

    #[test]
    fn test_run_executes_future_and_returns_ok() {
        let result: Result<(), SchedulerError> =
            make_scheduler(TokioSchedulerConfig::default(), "t").run(async {});
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_passes_through_future_output() {
        let result: Result<i32, SchedulerError> =
            make_scheduler(TokioSchedulerConfig::default(), "t").run(async { 42 });
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_run_with_worker_count_succeeds() {
        use std::num::NonZeroUsize;
        let cfg = TokioSchedulerConfig {
            workers: NonZeroUsize::new(2),
            ..Default::default()
        };
        let result: Result<(), SchedulerError> = make_scheduler(cfg, "t").run(async {});
        assert!(result.is_ok());
    }
}
