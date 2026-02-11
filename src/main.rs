mod cli;
mod config;

use crate::config::ConfigLoader;
use clap::Parser;
use cli::{CLI, Command};
use obsidian_tidy_lints::ALL_LINTS;
use obsidian_tidy_logging::LoggerBuilder;
use std::fs::OpenOptions;

fn main() -> anyhow::Result<()> {
    let args = CLI::parse();

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

            let mut file = OpenOptions::new().read(true).open(&config_path)?;
            let config = ConfigLoader::new(&ALL_LINTS).load(&mut file)?;
            anyhow::bail!("my config: {config:?}");
        }
    }

    Ok(())
}
