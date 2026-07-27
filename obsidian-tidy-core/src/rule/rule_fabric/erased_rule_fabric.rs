use crate::rule::{Category, ErasedRule, RuleFabric};
use serde::Serialize;
use std::fmt::Debug;

/// Erased version [`RuleFabric`]
pub trait ErasedRuleFabric {
    fn name_rule(&self) -> &str;

    fn description_rule(&self) -> &str;

    fn category_rule(&self) -> Category;

    fn create_rule(
        &self,
        deserializer: &mut dyn erased_serde::Deserializer,
    ) -> Result<Box<dyn ErasedRule>, Box<dyn std::error::Error>>;
}

impl<R> ErasedRuleFabric for R
where
    R: RuleFabric,
    <R as RuleFabric>::Rule: Serialize + 'static,
    <R as RuleFabric>::Error: 'static,
{
    fn name_rule(&self) -> &str {
        R::name_rule(self)
    }

    fn description_rule(&self) -> &str {
        R::description_rule(self)
    }

    fn category_rule(&self) -> Category {
        R::category_rule(self)
    }

    fn create_rule(
        &self,
        deserializer: &mut dyn erased_serde::Deserializer,
    ) -> Result<Box<dyn ErasedRule>, Box<dyn std::error::Error>> {
        let data: R::Data = erased_serde::deserialize(deserializer).map_err(Box::new)?;
        let rule = R::create_rule(self, data)?;

        Ok(Box::new(rule))
    }
}

pub trait IntoErasedRuleFabric {
    fn into_erased(self) -> Box<dyn ErasedRuleFabric + Send + Sync>;
}

impl<R> IntoErasedRuleFabric for R
where
    R: RuleFabric + Send + Sync + 'static,
    <R as RuleFabric>::Rule: Serialize,
{
    fn into_erased(self) -> Box<dyn ErasedRuleFabric + Send + Sync> {
        Box::new(self)
    }
}

impl<R> From<R> for Box<dyn ErasedRuleFabric>
where
    R: RuleFabric + Send + Sync + 'static,
    <R as RuleFabric>::Rule: Serialize,
{
    fn from(rule_fabric: R) -> Self {
        rule_fabric.into_erased()
    }
}

impl Debug for dyn ErasedRuleFabric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ErasedRuleFabric")
            .field("name_rule", &self.name_rule())
            .field("description_rule", &self.description_rule())
            .field("category_rule", &self.category_rule())
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        rule::RuleMetadata,
        test_utils::{TestRule, TestRuleFabric},
    };

    #[test]
    fn rule_erased() {
        const TEST_NAME: &str = "Test name";
        const TEST_DESCRIPTION: &str = "test description";
        const TEST_CATEGORY: Category = Category::Heading;

        let test_rule = TestRule::new(TEST_NAME, TEST_DESCRIPTION, TEST_CATEGORY, []);
        let json = serde_json::to_string_pretty(&test_rule).unwrap();

        let mut deserializer = serde_json::Deserializer::from_str(&json);
        let mut erased_deserializer = <dyn erased_serde::Deserializer>::erase(&mut deserializer);
        let fabric: Box<dyn ErasedRuleFabric> = Box::new(TestRuleFabric::new(
            TEST_NAME,
            TEST_DESCRIPTION,
            TEST_CATEGORY,
        ));

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
        let fabric: Box<dyn ErasedRuleFabric> = Box::new(TestRuleFabric::default());

        let result = fabric.create_rule(&mut erased_deserializer);
        assert!(result.is_err(), "Should fail with invalid JSON");

        if let Err(e) = result {
            tracing::info!("expected error: {e}");
        }
    }
}
