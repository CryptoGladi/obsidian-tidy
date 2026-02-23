use obsidian_tidy_cli::{Cli, LoggerConfig, Parser};
use obsidian_tidy_logging::{Logger, LoggerBuilder};

fn init_logger(logger_config: &LoggerConfig) -> Option<Logger> {
    if logger_config.enable_logger {
        let logger = LoggerBuilder::default()
            .filter(logger_config.log_level.into())
            .stdout(logger_config.enable_logger_stdout)
            .file(logger_config.enable_logger_file)
            .path(logger_config.path_log.clone())
            .init();

        return Some(logger);
    }

    None
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();
    let _logger = init_logger(&args.logger);

    let command = args.command;
    command.execute(&args)?;

    Ok(())
}
