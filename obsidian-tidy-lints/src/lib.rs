//! Module for Rust writer lints

use obsidian_tidy_core::lint::{Category, Content, Lint, Violation};

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
        "TestLint"
    }

    fn description(&self) -> &str {
        "Test lint"
    }

    fn category(&self) -> Category {
        Category::Custom
    }

    fn priority(&self) -> u32 {
        20
    }

    fn check(&self, _content: &Content) -> Vec<Violation> {
        Vec::new()
    }
}

pub struct Test1;

impl Lint for Test1 {
    fn name(&self) -> &str {
        "TestLint1"
    }

    fn description(&self) -> &str {
        "Test lint 1"
    }

    fn category(&self) -> Category {
        Category::Custom
    }

    fn priority(&self) -> u32 {
        20
    }

    fn check(&self, _content: &Content) -> Vec<Violation> {
        Vec::new()
    }
}
