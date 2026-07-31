use crate::{
    Note,
    rule::{
        Category, Content, RuleConstMetadata, RuleRunner, Violation,
        rule_fabric::GetFabricFromRuleConstMetadata,
    },
};
use core::iter::IntoIterator;
use serde::{Deserialize, Serialize};
use static_assertions::assert_impl_all;
use std::convert::Infallible;

/// Mock rule for the benchmark
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct HeadingSpacingRule {
    pub min_spaces: usize,
    pub max_spaces: usize,
}

assert_impl_all!(HeadingSpacingRule: GetFabricFromRuleConstMetadata);

impl RuleConstMetadata for HeadingSpacingRule {
    const NAME: &'static str = "heading-spacing";
    const DESCRIPTION: &'static str = "Checks spacing between headings";
    const CATEGORY: Category = Category::Spacing;
}

impl RuleRunner for HeadingSpacingRule {
    type Error = Infallible;
    fn check(&self, _content: &Content, _note: &Note) -> Result<Vec<Violation>, Self::Error> {
        Ok(vec![])
    }
}

/// Mock rule for the benchmark
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct DateFormatRule {
    pub format: String,
}

assert_impl_all!(DateFormatRule: GetFabricFromRuleConstMetadata);

impl RuleConstMetadata for DateFormatRule {
    const NAME: &'static str = "date-format";
    const DESCRIPTION: &'static str = "Validates date formats in YAML";
    const CATEGORY: Category = Category::Yaml;
}

impl RuleRunner for DateFormatRule {
    type Error = Infallible;
    fn check(&self, _content: &Content, _note: &Note) -> Result<Vec<Violation>, Self::Error> {
        Ok(vec![])
    }
}

/// Mock rule for the benchmark
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ForbiddenWordsRule {
    pub words: Vec<String>,
}

assert_impl_all!(ForbiddenWordsRule: GetFabricFromRuleConstMetadata);

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

impl RuleConstMetadata for ForbiddenWordsRule {
    const NAME: &'static str = "forbidden-words";
    const DESCRIPTION: &'static str = "Flags specific forbidden words";
    const CATEGORY: Category = Category::Content;
}

impl RuleRunner for ForbiddenWordsRule {
    type Error = Infallible;
    fn check(&self, _content: &Content, _note: &Note) -> Result<Vec<Violation>, Self::Error> {
        Ok(vec![])
    }
}

/// Mock rule for the benchmark
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct EmptyFileRule {
    pub check_size: bool,
    pub min_bytes: usize,
}

assert_impl_all!(EmptyFileRule: GetFabricFromRuleConstMetadata);

impl RuleConstMetadata for EmptyFileRule {
    const NAME: &'static str = "empty-file";
    const DESCRIPTION: &'static str = "Checks for empty or too small files";
    const CATEGORY: Category = Category::Other;
}

impl RuleRunner for EmptyFileRule {
    type Error = Infallible;
    fn check(&self, _content: &Content, _note: &Note) -> Result<Vec<Violation>, Self::Error> {
        Ok(vec![])
    }
}

/// Mock rule for the benchmark
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct TagConsistencyRule {
    pub required_tags: Vec<String>,
    pub allow_extra: bool,
}

assert_impl_all!(TagConsistencyRule: GetFabricFromRuleConstMetadata);

impl RuleConstMetadata for TagConsistencyRule {
    const NAME: &'static str = "tag-consistency";
    const DESCRIPTION: &'static str = "Ensures required tags are present";
    const CATEGORY: Category = Category::Yaml;
}

impl RuleRunner for TagConsistencyRule {
    type Error = Infallible;
    fn check(&self, _content: &Content, _note: &Note) -> Result<Vec<Violation>, Self::Error> {
        Ok(vec![])
    }
}
