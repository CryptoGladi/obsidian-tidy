//! Module for testing

use crate::rule::{Category, Content, Rule, Violation};
use std::convert::Infallible;

#[derive(Debug, PartialEq, Eq)]
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

    fn check(&self, _content: &Content) -> Result<Vec<Violation>, Self::Error> {
        Ok(self.check_result.clone())
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
