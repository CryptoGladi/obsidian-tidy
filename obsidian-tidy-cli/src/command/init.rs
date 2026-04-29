//! Command for init config file

use super::Cli;
use crate::command::runner::Runner;
use obsidian_tidy_config::template::Template;
use obsidian_tidy_config::{Config, ConfigSaver};
use std::fs::OpenOptions;
use std::path::PathBuf;
use thiserror::Error;
use tracing::{debug, instrument};

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Config problem: {0}")]
    Config(#[from] obsidian_tidy_config::Error),

    #[error("Config file already exists")]
    AlreadyExists(PathBuf),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunnerInit {
    override_config: bool,
    template: Template,
}

impl RunnerInit {
    pub const fn new(override_config: bool, template: Template) -> Self {
        Self {
            override_config,
            template,
        }
    }
}

impl Runner for RunnerInit {
    type Error = self::Error;

    #[instrument]
    fn run(&self, args: &Cli) -> Result<(), Self::Error> {
        debug!("run command `init`");

        let config_path = args.config();

        if config_path.is_file() {
            if self.override_config {
                std::fs::remove_file(&config_path)?;
            } else {
                return Err(Error::AlreadyExists(config_path));
            }
        }

        let config = Config {
            rules: self.template.into(),
        };

        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&config_path)?;

        ConfigSaver::new(&config).save(&mut file)?;

        Ok(())
    }
}
