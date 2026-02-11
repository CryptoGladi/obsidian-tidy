//! Crate for logging

mod builder;
mod logger;

const ENV_NAME: &str = "OBSIDIAN_TIDY_LOG";

pub use builder::LoggerBuilder;
