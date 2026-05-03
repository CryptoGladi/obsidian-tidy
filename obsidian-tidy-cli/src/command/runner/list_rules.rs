//! Get list all rules

use super::Runnable;
use crate::Cli;
use obsidian_tidy_config::template::Template;
use obsidian_tidy_core::rule::Rules;

#[derive(Debug)]
pub struct ListRules {
    template: Template,
}

impl ListRules {
    pub const fn new(template: Template) -> Self {
        Self { template }
    }
}

impl Runnable for ListRules {
    fn run(self, _cli: &Cli) -> miette::Result<()> {
        let rules = Rules::from(self.template);
        for _rule in rules.names() {
            // rule - String
        }

        todo!()
    }
}
