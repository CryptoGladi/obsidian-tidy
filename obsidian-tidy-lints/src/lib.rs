//! Module for Rust writer lints

pub mod content;
pub mod lint_collection;

use lint_collection::lint_collection;
use obsidian_tidy_core::lint::{Category, Content, Lint, SharedErrorLint, Violation};
use std::{convert::Infallible, sync::LazyLock};

pub const ALL_LINTS: LazyLock<Vec<SharedErrorLint>> = lint_collection![Test, Test1];

pub struct Test;

impl Lint for Test {
    type Error = Infallible;

    fn name(&self) -> &str {
        "test-lint"
    }

    fn description(&self) -> &str {
        "Test lint"
    }

    fn category(&self) -> Category {
        Category::Custom
    }

    fn check(&self, _content: &Content) -> Result<Vec<Violation>, Self::Error> {
        Ok(Vec::new())
    }
}

pub struct Test1;

impl Lint for Test1 {
    type Error = Infallible;

    fn name(&self) -> &str {
        "test-lint1"
    }

    fn description(&self) -> &str {
        "Test lint 1"
    }

    fn category(&self) -> Category {
        Category::Custom
    }

    fn check(&self, _content: &Content) -> Result<Vec<Violation>, Self::Error> {
        Ok(Vec::new())
    }
}
