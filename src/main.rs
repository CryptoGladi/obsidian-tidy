mod args;
mod config;

use args::{Cli, Command};
use clap::Parser;
use obsidian_tidy_logging::LoggerBuilder;

fn main() {
    let args = Cli::parse();

    let _logger = LoggerBuilder::default()
        .stdout(!args.quiet)
        .path(args.logs)
        .init();
}
