//! Module for impl [`Rule`] for smart pointer (Box and Arc)

use super::{Category, Content, Rule, Violation};
use std::sync::Arc;

impl<L> Rule for Box<L>
where
    L: Rule,
{
    type Error = L::Error;

    fn name(&self) -> &str {
        self.as_ref().name()
    }

    fn description(&self) -> &str {
        self.as_ref().description()
    }

    fn category(&self) -> Category {
        self.as_ref().category()
    }

    fn check(&self, content: &Content) -> Result<Vec<Violation>, Self::Error> {
        self.as_ref().check(content)
    }
}

impl<L> Rule for Arc<L>
where
    L: Rule,
{
    type Error = L::Error;

    fn name(&self) -> &str {
        self.as_ref().name()
    }

    fn description(&self) -> &str {
        self.as_ref().description()
    }

    fn category(&self) -> Category {
        self.as_ref().category()
    }

    fn check(&self, content: &Content) -> Result<Vec<Violation>, Self::Error> {
        self.as_ref().check(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::TestRule;
    use std::convert::Infallible;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn check_box() {
        let test_rule = TestRule::new("test-rule", "", Category::Heading);
        let test_rule = Box::new(test_rule) as Box<dyn Rule<Error = Infallible>>;

        assert_eq!(test_rule.name(), "test-rule");
    }

    #[test]
    #[traced_test]
    fn check_arc() {
        let test_rule = TestRule::new("test-rule", "", Category::Heading);
        let test_rule = Arc::new(test_rule) as Arc<dyn Rule<Error = Infallible>>;

        assert_eq!(test_rule.name(), "test-rule");
    }
}
