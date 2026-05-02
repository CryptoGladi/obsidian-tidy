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

        let mut file = if self.force {
            OpenOptions::new().write(true).truncate(true).open(&path)
        } else {
            OpenOptions::new().write(true).create_new(true).open(&path)
        }
        .into_diagnostic()
        .context(format!("open file in path: `{}`", path.display()))?;

        ConfigSaver::new(&config)
            .save(&mut file)
            .into_diagnostic()
            .context("serialize json")?;

        println!("✨ Config created at {}", path.display());
        Ok(())
    }
}
