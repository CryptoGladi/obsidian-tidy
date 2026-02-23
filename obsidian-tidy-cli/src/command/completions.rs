//! Generate shell completions

use super::runner::Runner;
use crate::Cli;
use clap::CommandFactory;
use std::{convert::Infallible, io};
use tracing::{debug, instrument};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunnerCompletions {
    shell: clap_complete::Shell,
}

impl RunnerCompletions {
    pub const fn new(shell: clap_complete::Shell) -> Self {
        Self { shell }
    }
}

impl Runner for RunnerCompletions {
    type Error = Infallible;

    #[instrument]
    fn run(&self, args: &Cli) -> Result<(), Self::Error> {
        debug!("Run completions command");

        clap_complete::generate(
            self.shell,
            &mut Cli::command(),
            "obsidian-tidy",
            &mut io::stdout(),
        );

        Ok(())
    }
}
