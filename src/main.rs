use miette::{Context, IntoDiagnostic};
use obsidian_tidy_cli::{Cli, LoggerConfig, Parser};
use obsidian_tidy_logger::{LoggerBuilder, WorkerGuard};

fn init_logger(logger_config: &LoggerConfig) -> miette::Result<Option<WorkerGuard>> {
    if logger_config.enable_logger {
        let guard = LoggerBuilder::default()
            .console_filter(logger_config.log_level.into())
            .file_filter(logger_config.log_level.into())
            .stdout(logger_config.enable_logger_stdout)
            .file(logger_config.enable_logger_file)
            .path(logger_config.path_log.clone())
            .build()
            .into_diagnostic()
            .context("Initializing logging")?
            .init()
            .into_diagnostic()
            .context("Initializing logging")?;

        return Ok(guard);
    }

    Ok(None)
}

fn main() -> miette::Result<()> {
    let args = Cli::parse();
    let _guard = init_logger(&args.logger)?;

    let command = args.command;
    command.execute(&args)?;

    Ok(())
}
