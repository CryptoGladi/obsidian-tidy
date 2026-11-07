mod logger;

use obsidian_tidy_cli::{Cli, Parser};

fn main() -> miette::Result<()> {
    let args = Cli::parse();
    let _guard = logger::init(&args.logger)?;

    let command = args.command;
    command.execute(&args)?;

    Ok(())
}
