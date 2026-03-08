use crate::rule::{Category, Rule, RuleFabric, StaticRule};
use serde::Deserialize;
use std::{convert::Infallible, marker::PhantomData};

pub trait GetFabricFromStaticRule {
    type Rule: Rule + for<'de> Deserialize<'de>;
    type StaticRule: StaticRule;

    fn fabric() -> impl RuleFabric<Rule = Self::Rule, Data = Self::Rule, Error = Infallible> {
        struct FabricFromRule<R: Rule + for<'de> Deserialize<'de>> {
            name_rule: &'static str,
            description_rule: &'static str,
            category: &'static Category,
            phantom: PhantomData<R>,
        }

        impl<R> RuleFabric for FabricFromRule<R>
        where
            R: Rule + for<'de> Deserialize<'de>,
        {
            type Rule = R;
            type Data = R;
            type Error = Infallible;

            fn name_rule(&self) -> &str {
                self.name_rule
            }

            fn description_rule(&self) -> &str {
                self.description_rule
            }

            fn category_rule(&self) -> Category {
                *self.category
            }

            fn create_rule(&self, data: Self::Data) -> Result<Self::Rule, Self::Error> {
                Ok(data)
            }
        }

        let fabric = FabricFromRule {
            name_rule: Self::StaticRule::NAME,
            description_rule: Self::StaticRule::DESCRIPTION,
            category: Self::StaticRule::CATEGORY,
            phantom: PhantomData,
        };

        fabric
    }
}

impl<R> GetFabricFromStaticRule for R
where
    R: Rule + StaticRule + for<'de> Deserialize<'de>,
{
    type Rule = R;
    type StaticRule = R;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize, Default)]
    struct TestRule;

    impl StaticRule for TestRule {
        const NAME: &'static str = "test-rule";
        const DESCRIPTION: &'static str = "It is test rule";
        const CATEGORY: &'static Category = &Category::Heading;
    }

    impl Rule for TestRule {
        type Error = Infallible;

        fn name(&self) -> &str {
            Self::NAME
        }

        fn description(&self) -> &str {
            Self::DESCRIPTION
        }

        fn category(&self) -> Category {
            *Self::CATEGORY
        }

        fn check(
            &self,
            _content: &crate::rule::Content,
            _note: &crate::Note,
        ) -> Result<Vec<crate::rule::Violation>, Self::Error> {
            Ok(Vec::new())
        }
    }

    #[test]
    fn fabric() {
        let rule = TestRule::default();
        let fabric = TestRule::fabric();

        assert_eq!(fabric.name_rule(), rule.name());
        assert_eq!(fabric.description_rule(), rule.description());
        assert_eq!(fabric.category_rule(), rule.category());

        fabric.create_rule(TestRule::default()).unwrap();
    }
}
