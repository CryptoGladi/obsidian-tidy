//! Builder for config

use super::Config;
use obsidian_tidy_core::rule::{Rules, SharedErrorRule};

#[derive(Debug)]
pub struct ConfigBuilder {
    rules: Rules<SharedErrorRule>,
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self {
            rules: Rules::new(Vec::new()).unwrap(),
        }
    }
}

impl ConfigBuilder {
    pub fn rules(mut self, rules: Rules<SharedErrorRule>) -> Self {
        self.rules = rules;
        self
    }

    pub fn build(self) -> Config {
        Config { rules: self.rules }
    }
}
