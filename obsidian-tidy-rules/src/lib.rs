//! Module for Rust writer rules

pub mod content;
pub mod rules;

#[cfg(test)]
pub(crate) mod test_utils;

use obsidian_tidy_core::rule::{Category, Content, Rule, SharedErrorRule, Violation};
use rules::rules;
use std::{convert::Infallible, sync::LazyLock};

pub static ALL_RULES: LazyLock<Vec<SharedErrorRule>> =
    rules![Test, Test1, content::empty_content::EmptyContent];

pub struct Test;

impl Rule for Test {
    type Error = Infallible;

    fn name(&self) -> &str {
        "test-rule"
    }

    fn description(&self) -> &str {
        "Test rule"
    }

    fn category(&self) -> Category {
        Category::Other
    }

    fn check(&self, _content: &Content) -> Result<Vec<Violation>, Self::Error> {
        Ok(Vec::new())
    }
}

pub struct Test1;

impl Rule for Test1 {
    type Error = Infallible;

    fn name(&self) -> &str {
        "test-rule1"
    }

    fn description(&self) -> &str {
        "Test rule 1"
    }

    fn category(&self) -> Category {
        Category::Other
    }

    fn check(&self, _content: &Content) -> Result<Vec<Violation>, Self::Error> {
        Ok(Vec::new())
    }
}
