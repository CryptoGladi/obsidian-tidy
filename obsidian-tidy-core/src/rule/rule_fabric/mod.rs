pub mod rule_fabric_registry;

pub use rule_fabric_registry::RuleFabricRegistry;

use crate::rule::{Category, Rule, SharedErrorRule};
use serde::Deserialize;
use std::convert::Infallible;

pub trait RuleFabric {
    type Rule: super::Rule;
    type Data: for<'de> Deserialize<'de>;
    type Error: std::error::Error;

    fn name_rule(&self) -> &str;

    fn description_rule(&self) -> &str;

    fn category_rule(&self) -> Category;

    fn create_rule(&self, data: Self::Data) -> Result<Self::Rule, Self::Error>;
}

impl<R> RuleFabric for R
where
    R: Rule + for<'de> Deserialize<'de>,
{
    type Rule = R;
    type Data = R;
    type Error = Infallible;

    fn name_rule(&self) -> &str {
        R::name(&self)
    }

    fn description_rule(&self) -> &str {
        R::description(&self)
    }

    fn category_rule(&self) -> Category {
        R::category(&self)
    }

    fn create_rule(&self, data: Self::Data) -> Result<Self::Rule, Self::Error> {
        Ok(data)
    }
}

pub trait ErasedRuleFabric {
    fn name_rule(&self) -> &str;

    fn description_rule(&self) -> &str;

    fn category_rule(&self) -> Category;

    fn create_rule(
        &self,
        deserializer: &mut dyn erased_serde::Deserializer,
    ) -> Result<SharedErrorRule, Box<dyn std::error::Error>>;
}

impl<R> ErasedRuleFabric for R
where
    R: RuleFabric,
    R::Rule: Send + Sync + 'static,
    <R::Rule as Rule>::Error: Send + Sync,
    R::Error: Send + Sync + 'static,
{
    fn name_rule(&self) -> &str {
        R::name_rule(&self)
    }

    fn description_rule(&self) -> &str {
        R::description_rule(&self)
    }

    fn category_rule(&self) -> Category {
        R::category_rule(&self)
    }

    fn create_rule(
        &self,
        deserializer: &mut dyn erased_serde::Deserializer,
    ) -> Result<SharedErrorRule, Box<dyn std::error::Error>> {
        let data: R::Data = erased_serde::deserialize(deserializer).map_err(Box::new)?;
        let rule = self.create_rule(data)?;

        Ok(SharedErrorRule::new(rule))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        rule::{Category, Rule},
        test_utils::TestRule,
    };
    use std::convert::Infallible;

    #[derive(Debug, Default)]
    struct TestRuleFabric;

    impl RuleFabric for TestRuleFabric {
        type Rule = TestRule;
        type Error = Infallible;
        type Data = TestRule;

        fn name_rule(&self) -> &str {
            "test-rule"
        }

        fn description_rule(&self) -> &str {
            "test description"
        }

        fn category_rule(&self) -> Category {
            Category::Heading
        }

        fn create_rule(&self, data: Self::Data) -> Result<Self::Rule, Self::Error> {
            Ok(data)
        }
    }

    #[test]
    fn test_rule_fabric() {
        const TEST_NAME: &str = "Test name";
        const TEST_DESCRIPTION: &str = "test description";
        const TEST_CATEGORY: Category = Category::Heading;

        let test_rule = TestRule::new(TEST_NAME, TEST_DESCRIPTION, TEST_CATEGORY, []);
        let json = serde_json::to_string_pretty(&test_rule).unwrap();

        let fabric = TestRuleFabric::default();
        let data = serde_json::from_str(&json).unwrap();
        let deserialized_rule = RuleFabric::create_rule(&fabric, data).unwrap();

        assert_eq!(test_rule, deserialized_rule);
        assert_eq!(test_rule.name(), deserialized_rule.name());
        assert_eq!(test_rule.description(), deserialized_rule.description());
        assert_eq!(test_rule.category(), deserialized_rule.category());
    }

    #[test]
    fn test_rule_erased() {
        const TEST_NAME: &str = "Test name";
        const TEST_DESCRIPTION: &str = "test description";
        const TEST_CATEGORY: Category = Category::Heading;

        let test_rule = TestRule::new(TEST_NAME, TEST_DESCRIPTION, TEST_CATEGORY, []);
        let json = serde_json::to_string_pretty(&test_rule).unwrap();

        let mut deserializer = serde_json::Deserializer::from_str(&json);
        let mut erased_deserializer = <dyn erased_serde::Deserializer>::erase(&mut deserializer);
        let fabric: Box<dyn ErasedRuleFabric> = Box::new(TestRuleFabric);

        let deserialized_rule = fabric
            .create_rule(&mut erased_deserializer)
            .expect("Failed to create rule from fabric");

        assert_eq!(test_rule.name(), deserialized_rule.name());
        assert_eq!(test_rule.description(), deserialized_rule.description());
        assert_eq!(test_rule.category(), deserialized_rule.category());
    }

    #[test]
    fn test_erased_deserialization_error() {
        let invalid_json = r#"{
        "name": "Broken Rule",
        "invalid_field": "should cause error"
    }"#;

        let mut deserializer = serde_json::Deserializer::from_str(invalid_json);
        let mut erased_deserializer = <dyn erased_serde::Deserializer>::erase(&mut deserializer);
        let fabric: Box<dyn ErasedRuleFabric> = Box::new(TestRuleFabric);

        let result = fabric.create_rule(&mut erased_deserializer);
        assert!(result.is_err(), "Should fail with invalid JSON");

        if let Err(e) = result {
            println!("Expected error: {}", e);
        }
    }
}
