//! Command for init config file

use super::Runnable;
use crate::Cli;
use miette::{Context, IntoDiagnostic};
use obsidian_tidy_config::Config;
use obsidian_tidy_config::template::Template;
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
    #[instrument(skip(args), level = "debug", err)]
    fn run(self, args: &Cli) -> miette::Result<()> {
        let path = args.config_path();

        let config = Config::new(self.template);

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

        Config::saver(&config)
            .pretty(true)
            .save(&mut file)
            .into_diagnostic()
            .context("Failed to serialize configuration to JSON")?;

        tracing::info!("✨ Config created at {}", path.display());
        Ok(())
    }
}
