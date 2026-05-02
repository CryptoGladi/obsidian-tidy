//! Command for init config file

use super::Runnable;
use crate::Cli;
use miette::{Context, IntoDiagnostic};
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
    fn run(self, args: &Cli) -> miette::Result<()> {
        let path = args.config_path();

        let config = Config {
            rules: self.template.into(),
        };

        let mut options = OpenOptions::new();
        options.write(true);

        if self.force {
            options.truncate(true);
        } else {
            options.create_new(true);
        }

        let mut file = options
            .open(&path)
            .into_diagnostic()
            .with_context(|| format!("Problem with file in: `{}`", path.display()))?;

        ConfigSaver::new(&config)
            .save(&mut file)
            .into_diagnostic()
            .context("Failed to serialize configuration to JSON")?;

        tracing::info!("✨ Config created at {}", path.display());
        Ok(())
    }
}
