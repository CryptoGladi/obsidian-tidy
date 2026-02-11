//! Module for Rust writer lints

pub mod content;

use obsidian_tidy_core::lint::{Category, Content, DynLint, Lint, Violation};
use std::sync::{Arc, LazyLock};

pub const ALL_LINTS: LazyLock<Vec<DynLint>> =
    LazyLock::new(|| vec![Arc::new(Test), Arc::new(Test1)]);

pub struct Test;

impl Lint for Test {
    fn name(&self) -> &str {
        "test-lint"
    }

    fn description(&self) -> &str {
        "Test lint"
    }

    fn category(&self) -> Category {
        Category::Custom
    }

    fn check(&self, _content: &Content) -> Vec<Violation> {
        Vec::new()
    }
}

pub struct Test1;

impl Lint for Test1 {
    fn name(&self) -> &str {
        "test-lint1"
    }

    fn description(&self) -> &str {
        "Test lint 1"
    }

    fn category(&self) -> Category {
        Category::Custom
    }

    fn check(&self, _content: &Content) -> Vec<Violation> {
        Vec::new()
    }
}
