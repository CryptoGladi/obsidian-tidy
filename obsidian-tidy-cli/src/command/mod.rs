mod check;
mod init;
mod list_rules;

use std::fs::OpenOptions;

use super::Cli;
use crate::command::{check::RunnerCheck, init::RunnerInit, list_rules::RunnerListRules};
use clap::Subcommand;
use obsidian_tidy_config::{loader::ConfigLoader, template::Template};
use obsidian_tidy_core::rule::Content;
use obsidian_tidy_rules::ALL_RULES;

#[derive(Debug, Subcommand)]
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
}

pub trait Runner {
    fn run(&self, args: &Cli) -> anyhow::Result<()>;
}

impl Command {
    pub fn execute(&self, args: &Cli) -> anyhow::Result<()> {
        let config_path = args.path.join(".obsidian-tidy.toml");

        let runner: &dyn Runner = match self {
            Command::Check => &RunnerCheck {
                content: &Content::default(),
                config: &ConfigLoader::new(&ALL_RULES)
                    .load(&mut OpenOptions::new().read(true).open(&config_path)?)?,
            },
            Command::Init {
                override_config,
                template,
            } => &RunnerInit {
                config_path,
                override_config: *override_config,
                template: *template,
            },
            Command::ListRules { from_template } => &RunnerListRules::new(from_template),
        };

        runner.run(args)
    }
}
