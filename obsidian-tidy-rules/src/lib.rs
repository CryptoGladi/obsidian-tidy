//! Module for Rust writer rules

#![forbid(clippy::print_stdout)]

pub mod content;
pub mod rules;

#[cfg(test)]
pub(crate) mod test_utils;

use obsidian_tidy_core::rule::SharedErrorRule;
use rules::rules;
use std::sync::LazyLock;

pub static ALL_RULES: LazyLock<Vec<SharedErrorRule>> = rules![content::empty_content::EmptyContent];
