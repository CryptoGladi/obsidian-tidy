//! Module for trait lints

use crate::Vault;
use std::ops::Range;

pub trait Lint: Send + Sync {
    /// Unique lint name
    fn name(&self) -> &'static str;

    fn description(&self) -> &'static str;

    fn category(&self) -> Category;

    fn priority(&self) -> u32;

    fn check(&self, content: &Content) -> Vec<Violation>;
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
