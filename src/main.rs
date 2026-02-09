mod args;
mod config;

use crate::{args::Template, config::Config};
use args::{Cli, Command};
use clap::Parser;
use obsidian_tidy_logging::LoggerBuilder;
use std::{fs::OpenOptions, path::Path};

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let _logger = LoggerBuilder::default()
        .stdout(!args.quiet)
        .path(args.logs.clone())
        .init();

    let config_path = args.path.join(".obsidian-tidy.toml");

    match args.command {
        Command::Init {
            override_config,
            template,
        } => config::init_command(&config_path, override_config, template)?,
        Command::Check => {
            if !config_path.is_file() {
                anyhow::bail!(
                    "Config file in `{}` not found\nRun `obsidian-tidy init`",
                    config_path.display()
                );
            }
        }
    }

    Ok(())
}
