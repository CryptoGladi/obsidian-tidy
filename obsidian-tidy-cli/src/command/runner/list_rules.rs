//! Get list all rules

use super::Runnable;
use crate::Cli;
use obsidian_tidy_config::template::Template;

#[derive(Debug)]
pub struct ListRules {
    from_template: Template,
}

impl ListRules {
    pub const fn new(from_template: Template) -> Self {
        Self { from_template }
    }
}

impl Runnable for ListRules {
    fn run(self, _cli: &Cli) -> anyhow::Result<()> {
        todo!()
    }
}
