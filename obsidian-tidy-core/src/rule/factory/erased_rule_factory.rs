use crate::rule::{ErasedRule, RuleFactory};
use serde::Serialize;

/// Erased version [`RuleFactory`]
pub trait ErasedRuleFactory {
    fn id(&self) -> &str;

    fn create_by_serde(
        &self,
        deserializer: &mut dyn erased_serde::Deserializer,
    ) -> Result<Box<dyn ErasedRule>, Box<dyn std::error::Error>>;

    fn create_default(&self) -> Option<Box<dyn ErasedRule>>;
}

static_assertions::assert_obj_safe!(ErasedRuleFactory);

impl<R> ErasedRuleFactory for R
where
    R: RuleFactory,
    <R as RuleFactory>::Rule: Serialize + 'static,
    <R as RuleFactory>::Error: 'static,
{
    fn id(&self) -> &str {
        R::id(self)
    }

    fn create_by_serde(
        &self,
        deserializer: &mut dyn erased_serde::Deserializer,
    ) -> Result<Box<dyn ErasedRule>, Box<dyn std::error::Error>> {
        let data: R::Data = erased_serde::deserialize(deserializer).map_err(Box::new)?;
        let rule = R::create_by_serde(self, data)?;

        Ok(Box::new(rule))
    }

    fn create_default(&self) -> Option<Box<dyn ErasedRule>> {
        R::create_default(self).map(|r| Box::new(r) as Box<dyn ErasedRule>)
    }
}

impl ErasedRuleFactory for &(dyn ErasedRuleFactory + Send + Sync) {
    fn id(&self) -> &str {
        (**self).id()
    }

    fn create_by_serde(
        &self,
        deserializer: &mut dyn erased_serde::Deserializer,
    ) -> Result<Box<dyn ErasedRule>, Box<dyn std::error::Error>> {
        (**self).create_by_serde(deserializer)
    }

    fn create_default(&self) -> Option<Box<dyn ErasedRule>> {
        (**self).create_default()
    }
}

pub trait IntoErasedRuleFactory {
    fn into_erased(self) -> Box<dyn ErasedRuleFactory + Send + Sync>;
}

impl<R> IntoErasedRuleFactory for R
where
    R: RuleFactory + Send + Sync + 'static,
    <R as RuleFactory>::Rule: Serialize,
{
    fn into_erased(self) -> Box<dyn ErasedRuleFactory + Send + Sync> {
        Box::new(self)
    }
}

impl<R> From<R> for Box<dyn ErasedRuleFactory>
where
    R: RuleFactory + Send + Sync + 'static,
    <R as RuleFactory>::Rule: Serialize,
{
    fn from(rule_fabric: R) -> Self {
        rule_fabric.into_erased()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        rule::{Category, RuleMetadata},
        test_utils::{TestRule, TestRuleFactory},
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
        let fabric: Box<dyn ErasedRuleFactory> = Box::new(TestRuleFactory::new(TEST_NAME));

        let deserialized_rule = fabric
            .create_by_serde(&mut erased_deserializer)
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
        let fabric: Box<dyn ErasedRuleFactory> = Box::new(TestRuleFactory::default());

        let result = fabric.create_by_serde(&mut erased_deserializer);
        assert!(result.is_err(), "Should fail with invalid JSON");

        if let Err(e) = result {
            tracing::info!("expected error: {e}");
        }
    }
}
