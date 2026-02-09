//! Module for Rust writer lints

pub mod content;

use obsidian_tidy_core::lint::{Category, Content, Lint, Violation};

pub const ALL_LINTS: &[&'static dyn Lint] = &[&Test, &Test1];

pub mod template {
    use super::*;
    use obsidian_tidy_core::lint::{Lints, ToggleableLint};

    pub fn empty() -> Lints {
        Lints::new(vec![
            ToggleableLint::with_enabled(Box::new(Test), false),
            ToggleableLint::with_enabled(Box::new(Test1), false),
        ])
        .unwrap()
    }

    pub fn standart() -> Lints {
        Lints::new(vec![
            ToggleableLint::with_enabled(Box::new(Test), true),
            ToggleableLint::with_enabled(Box::new(Test1), false),
        ])
        .unwrap()
    }

    pub fn all() -> Lints {
        Lints::new(vec![
            ToggleableLint::with_enabled(Box::new(Test), true),
            ToggleableLint::with_enabled(Box::new(Test1), true),
        ])
        .unwrap()
    }
}

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
