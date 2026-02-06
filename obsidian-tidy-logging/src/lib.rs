use obsidian_tidy_core::directories::directories;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{self, EnvFilter, prelude::*};

/// Init logger
pub fn init() -> WorkerGuard {
    let env_filter =
        EnvFilter::try_from_env("OBSIDIAN_TIDY_LOG").unwrap_or_else(|_| EnvFilter::new("off"));

    let console_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stdout)
        .with_target(false)
        .with_filter(env_filter.clone());

    let logs_dir = directories().logs_dir();
    let file_appender = tracing_appender::rolling::daily(logs_dir, "");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_ansi(false)
        .with_writer(non_blocking)
        .with_target(true)
        .with_line_number(true)
        .with_file(true)
        .with_filter(env_filter);

    tracing_subscriber::registry()
        .with(file_layer)
        .with(console_layer)
        .init();

    return guard;
}
