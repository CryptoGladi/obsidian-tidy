//! Command for init config file

use super::Runnable;
use crate::Cli;
use obsidian_tidy_config::template::Template;
use obsidian_tidy_config::{Config, ConfigSaver};
use std::fs::OpenOptions;
use tracing::instrument;

#[derive(Debug)]
pub struct Init {
    force: bool,
    template: Template,
}

impl Init {
    pub const fn new(force: bool, template: Template) -> Self {
        Self { force, template }
    }
}

impl Runnable for Init {
    #[instrument(skip(args), level = "info", err)]
    fn run(self, args: &Cli) -> anyhow::Result<()> {
        let config_path = args.config();

        let config = Config {
            rules: self.template.into(),
        };

        let mut file = if self.force {
            OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&config_path)
        } else {
            OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&config_path)
        }?;

        ConfigSaver::new(&config).save(&mut file)?;

        println!("✨ Config created at {}", config_path.display());
        Ok(())
    }
}
