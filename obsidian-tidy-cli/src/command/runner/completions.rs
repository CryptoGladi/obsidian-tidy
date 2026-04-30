//! Generate shell completions

use super::Runnable;
use crate::Cli;
use clap::CommandFactory;
use clap_complete::{Shell, generate};
use std::io;
use tracing::instrument;

#[derive(Debug)]
pub struct Completions {
    shell: Shell,
}

impl Completions {
    pub fn new(shell: Shell) -> Self {
        Self { shell }
    }
}

impl Runnable for Completions {
    #[instrument(skip_all, fields(shell = ?self.shell), level = "info", err)]
    fn run(self, _cli: &Cli) -> anyhow::Result<()> {
        generate(
            self.shell,
            &mut Cli::command(),
            "obsidian-tidy",
            &mut io::stdout(),
        );

        Ok(())
    }
}
