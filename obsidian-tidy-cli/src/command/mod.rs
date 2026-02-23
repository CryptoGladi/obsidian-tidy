mod check;
mod completions;
mod init;
mod list_rules;
mod runner;

use super::Cli;
use crate::command::{
    check::RunnerCheck, completions::RunnerCompletions, init::RunnerInit,
    list_rules::RunnerListRules, runner::SharedRunner,
};
use clap::Subcommand;
use obsidian_tidy_config::template::Template;
use std::sync::Arc;
use tracing::{debug, instrument};

#[derive(Debug, Clone, Copy, Subcommand)]
pub enum Command {
    /// Run rules
    Check,

    /// Initialization of config for obsidian-tidy
    Init {
        /// Override config if already exists
        #[arg(long = "override")]
        override_config: bool,

        /// How template use?
        #[arg(long, value_enum, default_value_t = Template::Standard)]
        template: Template,
    },

    /// List all available built‑in rules
    ListRules {
        /// Get rules from template
        #[arg(long, value_enum, default_value_t = Template::All)]
        from_template: Template,
    },

    /// Generate shell completions
    Completions {
        /// Shell type
        #[arg(value_enum)]
        shell: clap_complete::Shell,
    },
}

impl Command {
    /// Run command
    #[instrument(skip(args))]
    pub fn execute(self, args: &Cli) -> Result<(), Arc<dyn std::error::Error + Send + Sync>> {
        debug!("Execute command");

        let runner: SharedRunner = match self {
            Command::Check => RunnerCheck::new().into(),
            Command::Init {
                override_config,
                template,
            } => RunnerInit::new(override_config, template).into(),
            Command::ListRules { from_template } => RunnerListRules::new(from_template).into(),
            Command::Completions { shell } => RunnerCompletions::new(shell).into(),
        };

        runner.run(args)
    }
}
