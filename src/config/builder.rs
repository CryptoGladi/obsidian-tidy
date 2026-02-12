//! Builder for config

use super::Config;
use obsidian_tidy_core::lint::{Lints, WrappedAnyhowError};

#[derive(Debug, Default)]
pub struct ConfigBuilder {
    lints: Lints<WrappedAnyhowError>,
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self {
            lints: Lints::new(Vec::new()).unwrap(),
        }
    }
}

impl ConfigBuilder {
    pub fn lints(mut self, lints: Lints<WrappedAnyhowError>) -> Self {
        self.lints = lints;
        self
    }

    pub fn build(self) -> Config {
        Config { lints: self.lints }
    }
}
