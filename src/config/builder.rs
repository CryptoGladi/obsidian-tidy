//! Builder for config

use super::Config;
use obsidian_tidy_core::lint::{Lints, SharedErrorLint};

#[derive(Debug)]
pub struct ConfigBuilder {
    lints: Lints<SharedErrorLint>,
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self {
            lints: Lints::new(Vec::new()).unwrap(),
        }
    }
}

impl ConfigBuilder {
    pub fn lints(mut self, lints: Lints<SharedErrorLint>) -> Self {
        self.lints = lints;
        self
    }

    pub fn build(self) -> Config {
        Config { lints: self.lints }
    }
}
