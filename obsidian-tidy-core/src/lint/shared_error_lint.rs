use super::{Category, Content, Lint, Violation};
use crate::lint::DynLint;
use std::{ops::Deref, sync::Arc};

#[derive(Debug, Clone)]
pub struct SharedErrorLint {
    inner: DynLint<Arc<dyn std::error::Error>>,
}

impl SharedErrorLint {
    pub fn new<L>(lint: L) -> Self
    where
        L: Lint + 'static,
        L::Error: 'static,
    {
        let boxed = Arc::new(ErasingLint(lint));
        Self { inner: boxed }
    }
}

impl Lint for SharedErrorLint {
    type Error = Arc<dyn std::error::Error>;

    fn name(&self) -> &str {
        self.inner.name()
    }

    fn description(&self) -> &str {
        self.inner.description()
    }

    fn category(&self) -> Category {
        self.inner.category()
    }

    fn check(&self, content: &Content) -> Result<Vec<Violation>, Self::Error> {
        self.inner.check(content)
    }
}

impl Deref for SharedErrorLint {
    type Target = DynLint<Arc<dyn std::error::Error>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl PartialEq for SharedErrorLint {
    fn eq(&self, other: &Self) -> bool {
        &self.inner == &other.inner
    }
}

impl Eq for SharedErrorLint {}

struct ErasingLint<L: Lint>(L);

impl<L> From<L> for ErasingLint<L>
where
    L: Lint,
{
    fn from(value: L) -> Self {
        Self(value)
    }
}

impl<L> Lint for ErasingLint<L>
where
    L: Lint,
    L::Error: 'static,
{
    type Error = Arc<dyn std::error::Error>;

    fn name(&self) -> &str {
        self.0.name()
    }
    fn description(&self) -> &str {
        self.0.description()
    }
    fn category(&self) -> Category {
        self.0.category()
    }

    fn check(&self, content: &Content) -> Result<Vec<Violation>, Self::Error> {
        self.0
            .check(content)
            .map_err(|e| Arc::new(e) as Arc<dyn std::error::Error>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::TestLint;
    use thiserror::Error;
    use tracing_test::traced_test;

    #[derive(Debug, Error, PartialEq, Eq)]
    enum Error {
        #[error("Oh no...")]
        OhNo,
    }

    struct ErrorLint;

    impl Lint for ErrorLint {
        type Error = self::Error;

        fn name(&self) -> &str {
            "error-lint"
        }

        fn description(&self) -> &str {
            ""
        }

        fn category(&self) -> Category {
            Category::Custom
        }

        fn check(&self, _content: &Content) -> Result<Vec<Violation>, Self::Error> {
            Err(self::Error::OhNo)
        }
    }

    #[test]
    #[traced_test]
    fn new() {
        let test_lint = TestLint::new("test-ling", "", Category::Custom);
        let error_lint = ErrorLint;

        let lints = vec![
            SharedErrorLint::new(test_lint),
            SharedErrorLint::new(error_lint),
        ];

        let content = Content::default();
        let error = lints
            .into_iter()
            .find_map(|lint| lint.check(&content).err());

        assert_eq!(error.unwrap().downcast_ref(), Some(&self::Error::OhNo));
    }

    #[test]
    #[traced_test]
    fn erasing_lint() {
        let test_lint = TestLint::new("test-ling", "", Category::Custom);
        let error_lint = ErrorLint;

        let test_lint = ErasingLint::from(test_lint);
        let error_lint = ErasingLint::from(error_lint);

        let content = Content::default();

        assert_eq!(test_lint.check(&content).ok(), Some(Vec::new()));
        assert_eq!(
            error_lint.check(&content).err().unwrap().downcast_ref(),
            Some(&self::Error::OhNo)
        );
    }
}
