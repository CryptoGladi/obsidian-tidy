//! Module for Rust writer rules

#![forbid(clippy::print_stdout)]

pub mod content;

#[cfg(test)]
pub(crate) mod test_utils;

use obsidian_tidy_core::{
    rule::{RuleFabricRegistry, SharedErrorRule},
    rule_fabric_registry,
};
use std::sync::LazyLock;

pub static ALL_RULES: LazyLock<RuleFabricRegistry> =
    LazyLock::new(|| rule_fabric_registry![content::empty_content::fabric()]);
