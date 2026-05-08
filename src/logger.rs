use miette::Diagnostic;
use obsidian_tidy_cli::LoggerConfig;
use obsidian_tidy_logger::{LoggerBuilder, WorkerGuard};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("Logger initialization failed: {operation}")]
#[diagnostic(code(logger::init_failed), help("logger config: {config:#?}"))]
pub struct LoggerInitError {
    pub config: LoggerConfig,

    pub operation: String,

    #[source]
    pub source: Box<dyn std::error::Error + Send + Sync>,
}

#[cold]
fn make_init_error(
    config: &LoggerConfig,
    error: impl Into<Box<dyn std::error::Error + Send + Sync>>,
    operation: impl Into<String>,
) -> LoggerInitError {
    LoggerInitError {
        config: config.clone(),
        source: error.into(),
        operation: operation.into(),
    }
}

pub fn init(config: &LoggerConfig) -> miette::Result<WorkerGuard> {
    let enable_file = config.file_filter != "off";

    let guard = LoggerBuilder::default()
        .stdout(config.quiet)
        .file(enable_file)
        .path(&config.path_log)
        .file_filter(&config.file_filter)
        .map_err(|e| {
            make_init_error(
                config,
                e,
                format!("Parse file filter: `{}`", config.file_filter),
            )
        })?
        .console_filter(&config.console_filter)
        .map_err(|e| {
            make_init_error(
                config,
                e,
                format!("Parse console filter: `{}`", config.console_filter),
            )
        })?
        .build()
        .map_err(|e| make_init_error(config, e, "Building logger configuration"))?
        .init()
        .map_err(|e| make_init_error(config, e, "Activating logger subscriber"))?;

    Ok(guard)
}
