mod runner;

use clap::Subcommand;
use obsidian_tidy_config::template::Template;
use runner::RunnerCommand;

use crate::{Cli, command::runner::Runnable};

#[derive(Debug, Clone, Copy, Subcommand)]
pub enum Command {
    /// Run rules
    Check,

    /// Initialization of config for obsidian-tidy
    Init {
        /// Override config if already exists
        #[arg(long, short = 'f')]
        force: bool,

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
    pub fn execute(self, cli: &Cli) -> anyhow::Result<()> {
        let command_runner = RunnerCommand::from(self);
        command_runner.run(cli)
    }
}
