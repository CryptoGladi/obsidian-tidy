//! Crate for configuration

pub mod error;
pub mod loader;
pub mod saver;
pub mod template;

use obsidian_tidy_core::rule::Rules;
use serde::Serialize;
use thiserror::Error;

pub use error::Error;
pub use saver::ConfigSaver;

#[derive(Debug, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub rules: Rules,
}
