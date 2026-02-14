use obsidian_tidy_cli::{Cli, Parser};
use obsidian_tidy_logging::LoggerBuilder;

fn main() -> anyhow::Result<()> {
    better_panic::Settings::default()
        .message("obsidian-tidy panicked (crashed)")
        .install();

    let args = Cli::parse();

    let _logger = LoggerBuilder::default()
        .stdout(!args.quiet)
        .path(args.logs.clone())
        .init();

    args.command.execute(&args)?;
    Ok(())
}
