use super::{Cli, Runner};
use obsidian_tidy_config::ConfigSaver;
use obsidian_tidy_config::template::Template;
use obsidian_tidy_config::{builder::ConfigBuilder, error::Error as ConfigError};
use std::fs::OpenOptions;
use std::path::PathBuf;
use tracing::{debug, instrument};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunnerInit {
    pub(crate) config_path: PathBuf,
    pub(crate) override_config: bool,
    pub(crate) template: Template,
}

impl Runner for RunnerInit {
    #[instrument]
    fn run(&self, args: &Cli) -> anyhow::Result<()> {
        debug!("Run command `init`");

        if self.config_path.is_file() {
            match self.override_config {
                true => std::fs::remove_file(&self.config_path)?,
                false => return Err(ConfigError::AlreadyExists(self.config_path.to_path_buf()))?,
            };
        }

        let config = ConfigBuilder::default().rules(self.template.into()).build();

        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&self.config_path)?;

        ConfigSaver::new(&config)
            .path(&self.config_path)
            .save(&mut file)?;

        Ok(())
    }
}
