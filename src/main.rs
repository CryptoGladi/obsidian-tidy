use obsidian_tidy_cli::{Cli, Parser};
use obsidian_tidy_logging::{Logger, LoggerBuilder};

fn init_logger(args: &Cli) -> Option<Logger> {
    match args.disable_logger {
        false => Some(
            LoggerBuilder::default()
                .stdout(!args.quiet)
                .path(args.logs.clone())
                .init(),
        ),
        true => None,
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();

    let _logger = init_logger(&args);
    let command = args.command;
    command.execute(&args)?;

    Ok(())
}
