mod args;
mod config;

use std::fs::OpenOptions;

use args::{Cli, Command};
use clap::Parser;
use obsidian_tidy_logging::LoggerBuilder;

use crate::config::Config;

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let _logger = LoggerBuilder::default()
        .stdout(!args.quiet)
        .path(args.logs)
        .init();

    match args.command {
        Command::Init {
            override_config,
            template,
        } => {
            let config_path = args.path.join(".obsidian-tidy.toml");

            if config_path.is_file() && override_config {
                std::fs::remove_file(&config_path)?;
            }

            let mut file = OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(&config_path)?;

            Config::default().save(&mut file)?;
        }
        Command::Check { ignore_cache } => todo!(),
    }

    Ok(())
}
