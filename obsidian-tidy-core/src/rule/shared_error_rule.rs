use super::{Category, Content, Rule, Violation};
use crate::{Note, rule::DynRule};
use std::{ops::Deref, sync::Arc};

#[derive(Debug, Clone)]
pub struct SharedErrorRule {
    inner: DynRule<Arc<dyn std::error::Error + Send + Sync>>,
}

impl SharedErrorRule {
    pub fn new<R>(rule: R) -> Self
    where
        R: Rule + 'static,
        R::Error: Send + Sync + 'static,
    {
        let boxed = Arc::new(ErasingRule(rule));
        Self { inner: boxed }
    }
}

impl Rule for SharedErrorRule {
    type Error = Arc<dyn std::error::Error + Send + Sync>;

    #[inline]
    fn name(&self) -> &str {
        self.inner.name()
    }

    #[inline]
    fn description(&self) -> &str {
        self.inner.description()
    }

    #[inline]
    fn category(&self) -> Category {
        self.inner.category()
    }

    #[inline]
    fn check(&self, content: &Content, note: &Note) -> Result<Vec<Violation>, Self::Error> {
        self.inner.check(content, note)
    }
}

impl Deref for SharedErrorRule {
    type Target = DynRule<Arc<dyn std::error::Error + Send + Sync>>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl PartialEq for SharedErrorRule {
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl Eq for SharedErrorRule {}

struct ErasingRule<R: Rule>(R);

impl<R> From<R> for ErasingRule<R>
where
    R: Rule,
{
    fn from(value: R) -> Self {
        Self(value)
    }
}

impl<R> Rule for ErasingRule<R>
where
    R: Rule,
    R::Error: Send + Sync + 'static,
{
    type Error = Arc<dyn std::error::Error + Send + Sync>;

    #[inline]
    fn name(&self) -> &str {
        self.0.name()
    }

    #[inline]
    fn description(&self) -> &str {
        self.0.description()
    }

    #[inline]
    fn category(&self) -> Category {
        self.0.category()
    }

    fn check(&self, content: &Content, note: &Note) -> Result<Vec<Violation>, Self::Error> {
        self.0
            .check(content, note)
            .map_err(|e| Arc::new(e) as Arc<dyn std::error::Error + Send + Sync>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::TestRule;
    use thiserror::Error;
    use tracing_test::traced_test;

    #[derive(Debug, Error, PartialEq, Eq)]
    enum Error {
        #[error("Oh no...")]
        OhNo,
    }

    struct ErrorRule;

    impl Rule for ErrorRule {
        type Error = self::Error;

        fn name(&self) -> &'static str {
            "error-rule"
        }

        fn description(&self) -> &'static str {
            ""
        }

        fn category(&self) -> Category {
            Category::Other
        }

        fn check(&self, _content: &Content, _note: &Note) -> Result<Vec<Violation>, Self::Error> {
            Err(self::Error::OhNo)
        }
    }

    #[test]
    #[traced_test]
    fn new() {
        let test_rule = TestRule::new("test-rule", "", Category::Other, []);
        let error_rule = ErrorRule;

        let rules = vec![
            SharedErrorRule::new(test_rule),
            SharedErrorRule::new(error_rule),
        ];

        let content = Content::default();
        let error = rules
            .into_iter()
            .find_map(|rule| rule.check(&content, &Note::default()).err());

        assert_eq!(error.unwrap().downcast_ref(), Some(&self::Error::OhNo));
    }

    #[test]
    #[traced_test]
    fn erasing_rule() {
        let test_rule = TestRule::new("test-rule", "", Category::Other, []);
        let error_rule = ErrorRule;

        let test_rule = ErasingRule::from(test_rule);
        let error_rule = ErasingRule::from(error_rule);

        let content = Content::default();
        let note = Note::default();

        assert_eq!(test_rule.check(&content, &note).ok(), Some(Vec::new()));

        assert_eq!(
            error_rule
                .check(&content, &note)
                .err()
                .unwrap()
                .downcast_ref(),
            Some(&self::Error::OhNo)
        );
    }
}
