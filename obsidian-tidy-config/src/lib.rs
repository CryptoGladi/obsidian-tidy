//! Crate for configuration

pub mod error;
pub mod loader;
pub mod saver;
pub mod template;

pub use error::Error;
pub use loader::ConfigLoader;
pub use saver::ConfigSaver;

use obsidian_tidy_core::rule::Rules;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub rules: Rules,
}
