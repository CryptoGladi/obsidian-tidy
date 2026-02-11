//! Builder for config

use super::Config;
use obsidian_tidy_core::lint::Lints;

#[derive(Debug, Default)]
pub struct ConfigBuilder {
    lints: Lints,
}

impl ConfigBuilder {
    pub fn lints(mut self, lints: Lints) -> Self {
        self.lints = lints;
        self
    }

    pub fn build(self) -> Config {
        Config { lints: self.lints }
    }
}
