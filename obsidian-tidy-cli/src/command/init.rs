//! Command for init config file

use super::Cli;
use crate::command::runner::Runner;
use obsidian_tidy_config::ConfigSaver;
use obsidian_tidy_config::template::Template;
use obsidian_tidy_config::{builder::ConfigBuilder, error::Error as ConfigError};
use std::fs::OpenOptions;
use std::path::PathBuf;
use thiserror::Error;
use tracing::{debug, instrument};

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Config problem: {0}")]
    Config(#[from] ConfigError),

    #[error("Config file already exists")]
    AlreadyExists(PathBuf),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunnerInit {
    override_config: bool,
    template: Template,
}

impl RunnerInit {
    pub fn new(override_config: bool, template: Template) -> Self {
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
        debug!("Run command `init`");

        let config_path = args.config();

        if config_path.is_file() {
            match self.override_config {
                true => std::fs::remove_file(&config_path)?,
                false => return Err(Error::AlreadyExists(config_path)),
            };
        }

        let config = ConfigBuilder::default().rules(self.template.into()).build();

        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&config_path)?;

        ConfigSaver::new(&config)
            .path(&config_path)
            .save(&mut file)?;

        Ok(())
    }
}
