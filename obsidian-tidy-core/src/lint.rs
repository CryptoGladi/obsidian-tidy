//! Module for trait lints

use crate::Vault;
use serde::Serialize;
use std::{
    collections::HashSet,
    fmt::Debug,
    ops::{Deref, Range},
};
use thiserror::Error;

pub trait Lint: Send + Sync {
    /// Unique lint name
    fn name(&self) -> &str;

    fn description(&self) -> &str;

    fn category(&self) -> Category;

    fn priority(&self) -> u32;

    fn check(&self, content: &Content) -> Vec<Violation>;
}

impl Debug for dyn Lint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Lint")
            .field("name", &self.name())
            .field("description", &self.description())
            .field("category", &self.category())
            .field("priority", &self.priority())
            .finish_non_exhaustive()
    }
}

impl serde::Serialize for dyn Lint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.name())
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("Found duplicate name: {0}")]
    DuplicateName(String),
}

#[derive(Debug, Serialize)]
pub struct ToggleableLint {
    lint: Box<dyn Lint>,
    enabled: bool,
}

impl ToggleableLint {
    pub fn new(lint: Box<dyn Lint>) -> Self {
        Self {
            lint,
            enabled: true,
        }
    }

    pub fn with_enabled(lint: Box<dyn Lint>, enabled: bool) -> Self {
        Self { lint, enabled }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }
}

impl Deref for ToggleableLint {
    type Target = Box<dyn Lint>;

    fn deref(&self) -> &Self::Target {
        &self.lint
    }
}

#[derive(Debug, Serialize)]
pub struct Lints(Vec<ToggleableLint>);

impl Lints {
    fn check_unique_name(lints: &[ToggleableLint]) -> Result<(), Error> {
        let mut names = HashSet::with_capacity(lints.len());
        let iter = lints.iter().map(|lint| lint.name());

        for name in iter {
            if !names.insert(name) {
                return Err(Error::DuplicateName(name.to_string()));
            }
        }

        Ok(())
    }

    pub fn new(lints: Vec<ToggleableLint>) -> Result<Self, Error> {
        Self::check_unique_name(&lints)?;
        Ok(Self(lints))
    }
}

impl Deref for Lints {
    type Target = Vec<ToggleableLint>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub enum Category {
    Yaml,
    Heading,
    Content,
    Spacing,
    Custom,
}

#[derive(Debug, Clone)]
pub enum Severity {
    Allow,
    Warning,
    Deny,
}

#[derive(Debug, Clone)]
pub struct Content {
    vault: Vault,
}

#[derive(Debug, Clone)]
pub struct Violation {
    pub message: String,
    pub location: Range<usize>,
    pub severity: Severity,
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestLint {
        name: String,
    }

    impl TestLint {
        pub fn new(name: impl Into<String>) -> Self {
            Self { name: name.into() }
        }
    }

    impl Lint for TestLint {
        fn name(&self) -> &str {
            &self.name
        }

        fn description(&self) -> &str {
            unimplemented!()
        }

        fn category(&self) -> Category {
            unimplemented!()
        }

        fn priority(&self) -> u32 {
            unimplemented!()
        }

        fn check(&self, _content: &Content) -> Vec<Violation> {
            unimplemented!()
        }
    }

    #[test]
    fn duplicate_name() {
        let name = "DuplicateName";
        let lint1 = ToggleableLint::new(Box::new(TestLint::new(name)));
        let lint2 = ToggleableLint::new(Box::new(TestLint::new(name)));

        let lints = Lints::new(vec![lint1, lint2]);

        assert_eq!(lints.err(), Some(Error::DuplicateName(name.to_string())))
    }

    #[test]
    fn new() {
        let lint1 = ToggleableLint::new(Box::new(TestLint::new("Lint1")));
        let lint2 = ToggleableLint::new(Box::new(TestLint::new("Lint2")));

        let lints = Lints::new(vec![lint1, lint2]).unwrap();
        assert_eq!(lints.0.len(), 2);
    }
}
