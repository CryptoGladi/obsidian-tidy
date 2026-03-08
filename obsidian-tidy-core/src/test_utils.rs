//! Module for testing

use crate::{
    Note,
    rule::{Category, Content, Rule, RuleFabric, Violation},
};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) struct TestRule {
    name: String,
    description: String,
    category: Category,
    check_result: Vec<Violation>,
}

impl TestRule {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        category: Category,
        check_result: impl IntoIterator<Item = Violation>,
    ) -> TestRule {
        Self {
            name: name.into(),
            description: description.into(),
            category,
            check_result: check_result.into_iter().collect(),
        }
    }
}

impl Rule for TestRule {
    type Error = Infallible;

    fn name(&self) -> &str {
        &self.name
    }

    fn description(&self) -> &str {
        &self.description
    }

    fn category(&self) -> Category {
        self.category.clone()
    }

    fn check(&self, _content: &Content, _note: &Note) -> Result<Vec<Violation>, Self::Error> {
        Ok(self.check_result.clone())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct TestRuleFabric {
    name: String,
    description: String,
    category: Category,
}

impl TestRuleFabric {
    pub(crate) fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        category: Category,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            category,
        }
    }
}

impl RuleFabric for TestRuleFabric {
    type Rule = TestRule;
    type Data = TestRule;
    type Error = Infallible;

    fn name_rule(&self) -> &str {
        &self.name
    }

    fn description_rule(&self) -> &str {
        &self.description
    }

    fn category_rule(&self) -> Category {
        self.category
    }

    fn create_rule(&self, data: Self::Data) -> Result<Self::Rule, Self::Error> {
        debug_assert_eq!(data.name(), self.name_rule());
        debug_assert_eq!(data.description(), self.description_rule());
        debug_assert_eq!(data.category(), self.category_rule());

        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn new() {
        const TEST_NAME: &str = "Test name";
        const TEST_DESCRIPTION: &str = "test description";
        const TEST_CATEGORY: Category = Category::Heading;

        let test_rule = TestRule::new(TEST_NAME, TEST_DESCRIPTION, TEST_CATEGORY, []);

        assert_eq!(test_rule.name(), TEST_NAME);
        assert_eq!(test_rule.description(), TEST_DESCRIPTION);
        assert_eq!(test_rule.category(), TEST_CATEGORY);
    }
}
