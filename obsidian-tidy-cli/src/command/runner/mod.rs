mod check;
mod completions;
mod init;
mod list_rules;

use crate::{Cli, Command};

pub trait Runnable {
    fn run(self, cli: &Cli) -> anyhow::Result<()>;
}

#[derive(Debug)]
pub enum RunnerCommand {
    Check(check::Check),
    Init(init::Init),
    ListRules(list_rules::ListRules),
    Completions(completions::Completions),
}

impl Runnable for RunnerCommand {
    fn run(self, cli: &Cli) -> anyhow::Result<()> {
        match self {
            RunnerCommand::Check(check) => check.run(cli),
            RunnerCommand::Init(init) => init.run(cli),
            RunnerCommand::ListRules(list_rules) => list_rules.run(cli),
            RunnerCommand::Completions(completions) => completions.run(cli),
        }
    }
}

impl From<Command> for RunnerCommand {
    fn from(value: Command) -> Self {
        match value {
            Command::Check => Self::Check(check::Check),
            Command::Init { force, template } => Self::Init(init::Init::new(force, template)),
            Command::ListRules { from_template } => {
                Self::ListRules(list_rules::ListRules::new(from_template))
            }
            Command::Completions { shell } => {
                Self::Completions(completions::Completions::new(shell))
            }
        }
    }
}
