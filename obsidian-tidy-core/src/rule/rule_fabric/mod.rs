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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        rule::{Category, RuleMetadata},
        test_utils::{TestRule, TestRuleFabric},
    };

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
