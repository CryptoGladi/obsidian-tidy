use super::{Category, Content, Lint, Violation};
use std::sync::Arc;

impl<L> Lint for Box<L>
where
    L: Lint,
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

impl<L> Lint for Arc<L>
where
    L: Lint,
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
    use crate::test_utils::TestLint;
    use std::convert::Infallible;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn check_box() {
        let test_lint = TestLint::new("test-lint", "", Category::Heading);
        let test_lint = Box::new(test_lint) as Box<dyn Lint<Error = Infallible>>;

        assert_eq!(test_lint.name(), "test-lint");
    }

    #[test]
    #[traced_test]
    fn check_arc() {
        let test_lint = TestLint::new("test-lint", "", Category::Heading);
        let test_lint = Arc::new(test_lint) as Arc<dyn Lint<Error = Infallible>>;

        assert_eq!(test_lint.name(), "test-lint");
    }
}
