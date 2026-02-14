//! Crate for configuration

pub mod builder;
pub mod error;
pub mod loader;
pub mod saver;
pub mod template;

use obsidian_tidy_core::rule::{Rules, SharedErrorRule};
use serde::Serialize;
use thiserror::Error;

pub use error::Error;
pub use saver::ConfigSaver;

#[derive(Debug, Serialize)]
pub struct Config {
    rules: Rules<SharedErrorRule>,
}

impl Config {
    pub fn rules(&self) -> &Rules<SharedErrorRule> {
        &self.rules
    }
}
