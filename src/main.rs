use obsidian_tidy_cli::{Cli, Parser};
use obsidian_tidy_logging::LoggerBuilder;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();

    let _logger = LoggerBuilder::default()
        .stdout(!args.quiet)
        .path(args.logs.clone())
        .init();

    let command = args.command;
    command.execute(&args)?;

    Ok(())
}
