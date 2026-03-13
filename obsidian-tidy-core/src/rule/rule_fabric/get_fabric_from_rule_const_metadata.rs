use crate::rule::{Category, Rule, RuleConstMetadata, RuleFabric, RuleRunner};
use serde::Deserialize;
use std::{convert::Infallible, marker::PhantomData};

pub trait GetFabricFromRuleConstMetadata {
    type Rule: Rule + for<'de> Deserialize<'de>;
    type RuleConstMetadata: RuleConstMetadata;

    fn fabric() -> impl RuleFabric<Rule = Self::Rule, Data = Self::Rule, Error = Infallible> {
        struct FabricFromRule<R: Rule + for<'de> Deserialize<'de>> {
            name_rule: &'static str,
            description_rule: &'static str,
            category: Category,
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
                self.category
            }

            fn create_rule(&self, data: Self::Data) -> Result<Self::Rule, Self::Error> {
                Ok(data)
            }
        }

        FabricFromRule {
            name_rule: Self::RuleConstMetadata::NAME,
            description_rule: Self::RuleConstMetadata::DESCRIPTION,
            category: Self::RuleConstMetadata::CATEGORY,
            phantom: PhantomData,
        }
    }
}

impl<R> GetFabricFromRuleConstMetadata for R
where
    R: RuleRunner + RuleConstMetadata + for<'de> Deserialize<'de>,
{
    type Rule = R;
    type RuleConstMetadata = R;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule::{RuleMetadata, RuleRunner};

    #[derive(Deserialize, Default)]
    struct TestRule;

    impl RuleConstMetadata for TestRule {
        const NAME: &'static str = "test-rule";
        const DESCRIPTION: &'static str = "It is test rule";
        const CATEGORY: Category = Category::Heading;
    }

    impl RuleRunner for TestRule {
        type Error = Infallible;

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
