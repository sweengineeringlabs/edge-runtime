//! [`TokioScheduler`] — tokio-backed implementation of [`Scheduler`].

use std::future::Future;
use std::sync::OnceLock;

use crate::api::scheduler::scheduler_error::SchedulerError;
use crate::api::scheduler::tokio_scheduler_config::TokioSchedulerConfig;
use crate::api::scheduler::Scheduler;

/// Drives the process runtime using a tokio multi-thread scheduler.
pub(crate) struct TokioScheduler {
    config: TokioSchedulerConfig,
    thread_name: String,
}

impl TokioScheduler {
    pub(crate) fn new(config: TokioSchedulerConfig, thread_name: impl Into<String>) -> Self {
        let thread_name = config
            .thread_name
            .clone()
            .unwrap_or_else(|| thread_name.into());
        Self {
            config,
            thread_name,
        }
    }
}

static PANIC_HOOK: OnceLock<()> = OnceLock::new();

fn install_panic_hook() {
    PANIC_HOOK.get_or_init(|| {
        let prev = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            tracing::error!(panic = %info, "worker thread panicked — process will abort");
            prev(info);
        }));
    });
}

impl Scheduler for TokioScheduler {
    fn run<F, T>(&self, fut: F) -> Result<T, SchedulerError>
    where
        F: Future<Output = T> + Send + 'static,
    {
        install_panic_hook();

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

    #[test]
    fn test_new_uses_config_thread_name_when_set() {
        let cfg = TokioSchedulerConfig {
            thread_name: Some("cfg".into()),
            ..Default::default()
        };
        assert_eq!(TokioScheduler::new(cfg, "arg").thread_name, "cfg");
    }

    #[test]
    fn test_new_falls_back_to_arg_when_config_thread_name_absent() {
        assert_eq!(
            TokioScheduler::new(TokioSchedulerConfig::default(), "arg").thread_name,
            "arg"
        );
    }

    #[test]
    fn test_run_executes_future_and_returns_ok() {
        let result: Result<(), SchedulerError> =
            TokioScheduler::new(TokioSchedulerConfig::default(), "t").run(async { () });
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_passes_through_future_output() {
        let result: Result<i32, SchedulerError> =
            TokioScheduler::new(TokioSchedulerConfig::default(), "t").run(async { 42 });
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_run_with_worker_count_succeeds() {
        use std::num::NonZeroUsize;
        let cfg = TokioSchedulerConfig {
            workers: NonZeroUsize::new(2),
            ..Default::default()
        };
        let result: Result<(), SchedulerError> = TokioScheduler::new(cfg, "t").run(async { () });
        assert!(result.is_ok());
    }
}
