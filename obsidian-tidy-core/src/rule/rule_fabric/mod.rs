pub mod erased_rule_fabric;
pub mod get_fabric_from_rule_const_metadata;
pub mod rule_fabric_registry;

pub use erased_rule_fabric::ErasedRuleFabric;
pub use get_fabric_from_rule_const_metadata::GetFabricFromRuleConstMetadata;
pub use rule_fabric_registry::RuleFabricRegistry;

use crate::rule::Category;
use serde::Deserialize;

pub trait RuleFabric {
    type Rule: super::Rule;
    type Data: for<'de> Deserialize<'de>;
    type Error: std::error::Error;

    fn name_rule(&self) -> &str;

    fn description_rule(&self) -> &str;

    fn category_rule(&self) -> Category;

    fn create_rule(&self, data: Self::Data) -> Result<Self::Rule, Self::Error>;

    fn create_default_rule() -> Self::Rule
    where
        Self::Rule: Default,
    {
        Self::Rule::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        rule::{Category, RuleMetadata, RuleRunner},
        test_utils::{TestRule, TestRuleFabric},
    };
    use std::convert::Infallible;

    #[test]
    fn create_default_rule() {
        #[derive(Debug, Deserialize, PartialEq, Eq)]
        struct DefaultRule {
            name: String,
            description: String,
            category: Category,
        }

        impl Default for DefaultRule {
            fn default() -> Self {
                Self {
                    name: "default-rule".to_string(),
                    description: "Default rule".to_string(),
                    category: Category::Content,
                }
            }
        }

        impl RuleMetadata for DefaultRule {
            fn name(&self) -> &str {
                &self.name
            }

            fn description(&self) -> &str {
                &self.description
            }

            fn category(&self) -> Category {
                self.category
            }
        }

        impl RuleRunner for DefaultRule {
            type Error = Infallible;

            fn check(
                &self,
                _content: &crate::rule::Content,
                _note: &crate::Note,
            ) -> Result<Vec<crate::rule::Violation>, Self::Error> {
                Ok(Vec::new())
            }
        }

        struct DefaultRuleFabric;

        impl RuleFabric for DefaultRuleFabric {
            type Rule = DefaultRule;
            type Data = DefaultRule;
            type Error = Infallible;

            fn name_rule(&self) -> &str {
                "default-rule"
            }

            fn description_rule(&self) -> &str {
                "Default rule"
            }

            fn category_rule(&self) -> Category {
                Category::Content
            }

            fn create_rule(&self, data: Self::Data) -> Result<Self::Rule, Self::Error> {
                Ok(data)
            }
        }

        let rule = <DefaultRuleFabric as RuleFabric>::create_default_rule();

        assert_eq!(rule, DefaultRule::default());
    }

    #[test]
    fn test_rule_fabric() {
        const TEST_NAME: &str = "Test name";
        const TEST_DESCRIPTION: &str = "test description";
        const TEST_CATEGORY: Category = Category::Heading;

        let test_rule = TestRule::new(TEST_NAME, TEST_DESCRIPTION, TEST_CATEGORY, []);
        let json = serde_json::to_string_pretty(&test_rule).unwrap();

        let fabric = TestRuleFabric::new(TEST_NAME, TEST_DESCRIPTION, TEST_CATEGORY);
        let data = serde_json::from_str(&json).unwrap();
        let deserialized_rule = RuleFabric::create_rule(&fabric, data).unwrap();

        assert_eq!(test_rule, deserialized_rule);
        assert_eq!(test_rule.name(), deserialized_rule.name());
        assert_eq!(test_rule.description(), deserialized_rule.description());
        assert_eq!(test_rule.category(), deserialized_rule.category());
    }
}
