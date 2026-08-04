use crate::{
    Note,
    rule::{
        Category, Content, RuleMetadata, RuleRunner, Violation,
        factory::impl_rule_factory_with_serde,
    },
};
use core::iter::IntoIterator;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

/// Mock rule for the benchmark
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct HeadingSpacingRule {
    pub min_spaces: usize,
    pub max_spaces: usize,
}

impl RuleMetadata for HeadingSpacingRule {
    fn name(&self) -> &str {
        "heading-spacing"
    }

    fn description(&self) -> &str {
        "Checks spacing between headings"
    }

    fn category(&self) -> Category {
        Category::Spacing
    }
}

impl RuleRunner for HeadingSpacingRule {
    type Error = Infallible;
    fn check(&self, _content: &Content, _note: &Note) -> Result<Vec<Violation>, Self::Error> {
        Ok(vec![])
    }
}

pub struct HeadingSpacingFactory;
impl_rule_factory_with_serde!(HeadingSpacingFactory, HeadingSpacingRule, "heading-spacing");

/// Mock rule for the benchmark
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct DateFormatRule {
    pub format: String,
}

impl RuleMetadata for DateFormatRule {
    fn name(&self) -> &str {
        "date-format"
    }

    fn description(&self) -> &str {
        "Validates date formats in YAML"
    }

    fn category(&self) -> Category {
        Category::Yaml
    }
}

impl RuleRunner for DateFormatRule {
    type Error = Infallible;
    fn check(&self, _content: &Content, _note: &Note) -> Result<Vec<Violation>, Self::Error> {
        Ok(vec![])
    }
}

pub struct DateFormatFactory;
impl_rule_factory_with_serde!(DateFormatFactory, DateFormatRule, "date-format");

/// Mock rule for the benchmark
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ForbiddenWordsRule {
    pub words: Vec<String>,
}

impl ForbiddenWordsRule {
    pub fn new<I, S>(words: I) -> Self
    where
        I: IntoIterator<Item = S>,
        String: From<S>,
    {
        Self {
            words: words.into_iter().map(String::from).collect(),
        }
    }
}

impl RuleMetadata for ForbiddenWordsRule {
    fn name(&self) -> &str {
        "forbidden-words"
    }

    fn description(&self) -> &str {
        "Flags specific forbidden words"
    }

    fn category(&self) -> Category {
        Category::Content
    }
}

impl RuleRunner for ForbiddenWordsRule {
    type Error = Infallible;
    fn check(&self, _content: &Content, _note: &Note) -> Result<Vec<Violation>, Self::Error> {
        Ok(vec![])
    }
}

pub struct ForbiddenWordsFactory;
impl_rule_factory_with_serde!(ForbiddenWordsFactory, ForbiddenWordsRule, "forbidden-words");

/// Mock rule for the benchmark
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct EmptyFileRule {
    pub check_size: bool,
    pub min_bytes: usize,
}

impl RuleMetadata for EmptyFileRule {
    fn name(&self) -> &str {
        "empty-file"
    }

    fn description(&self) -> &str {
        "Checks for empty or too small files"
    }

    fn category(&self) -> Category {
        Category::Other
    }
}

impl RuleRunner for EmptyFileRule {
    type Error = Infallible;
    fn check(&self, _content: &Content, _note: &Note) -> Result<Vec<Violation>, Self::Error> {
        Ok(vec![])
    }
}

pub struct EmptyFileFactory;
impl_rule_factory_with_serde!(EmptyFileFactory, EmptyFileRule, "empty-file");

/// Mock rule for the benchmark
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct TagConsistencyRule {
    pub required_tags: Vec<String>,
    pub allow_extra: bool,
}

impl RuleMetadata for TagConsistencyRule {
    fn name(&self) -> &str {
        "tag-consistency"
    }

    fn description(&self) -> &str {
        "Ensures required tags are present"
    }

    fn category(&self) -> Category {
        Category::Yaml
    }
}

impl RuleRunner for TagConsistencyRule {
    type Error = Infallible;
    fn check(&self, _content: &Content, _note: &Note) -> Result<Vec<Violation>, Self::Error> {
        Ok(vec![])
    }
}

pub struct TagConsistencyFactory;
impl_rule_factory_with_serde!(TagConsistencyFactory, TagConsistencyRule, "tag-consistency");
