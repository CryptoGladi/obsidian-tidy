pub mod mock_rules;

use crate::{
    bench_utils::mock_rules::HeadingSpacingFactory,
    rule::{
        Category, ErasedRule, ErasedRuleFactory, ErasedRuleRunner, RuleFactoryRegistry,
        RuleMetadata, factory::erased_rule_factory::IntoErasedRuleFactory,
    },
};
use core::marker::Sync;
use mock_rules::{
    DateFormatFactory, DateFormatRule, EmptyFileFactory, EmptyFileRule, ForbiddenWordsFactory,
    ForbiddenWordsRule, HeadingSpacingRule, TagConsistencyFactory, TagConsistencyRule,
};
use std::collections::BTreeMap;

fn generate_name_by_index(i: usize) -> String {
    let idx = i % 5;
    format!("rule-type-{idx}-{i}")
}

/// Generates a JSON configuration with a specified number of rules.
/// The rules cycle through 5 types to ensure realism.
#[must_use]
#[expect(clippy::missing_panics_doc, reason = "It is only test code")]
#[expect(clippy::expect_used, reason = "It is only test code")]
pub fn generate_benchmark_json(rule_count: usize) -> String {
    let mut final_map: BTreeMap<String, serde_json::Value> = BTreeMap::new();

    for i in 0..rule_count {
        let idx = i % 5;
        let name = generate_name_by_index(i);

        let val = match idx {
            0 => serde_json::json!({
                "enable": true,
                "config": HeadingSpacingRule { min_spaces: 2, max_spaces: 4 }
            }),
            1 => serde_json::json!({
                "enable": false,
                "config": DateFormatRule { format: "%Y-%m-%d".to_string() }
            }),
            2 => serde_json::json!({
                "enable": true,
                "config": ForbiddenWordsRule::new(["foo", "bar"])
            }),
            3 => serde_json::json!({
                "enable": true,
                "config": EmptyFileRule { check_size: true, min_bytes: 50 }
            }),
            4 => serde_json::json!({
                "enable": false,
                "config": TagConsistencyRule { required_tags: vec!["todo".to_string()], allow_extra: true }
            }),
            _ => unreachable!(),
        };

        final_map.insert(name, val);
    }

    serde_json::to_string_pretty(&final_map).expect("json serializing")
}

struct FixNameRuleFactory {
    inner: Box<dyn ErasedRuleFactory + Send + Sync + 'static>,
    new_name: String,
}

impl FixNameRuleFactory {
    pub fn new(
        inner: Box<dyn ErasedRuleFactory + Send + Sync + 'static>,
        new_name: impl Into<String>,
    ) -> Self {
        Self {
            inner,
            new_name: new_name.into(),
        }
    }
}

impl ErasedRuleFactory for FixNameRuleFactory {
    fn name(&self) -> &str {
        &self.new_name
    }

    fn create_by_serde(
        &self,
        deserializer: &mut dyn erased_serde::Deserializer,
    ) -> Result<Box<dyn crate::rule::ErasedRule>, Box<dyn std::error::Error>> {
        let rule = self.inner.create_by_serde(deserializer)?;

        // If we do not change the name within the rule itself, we violate a
        // structural invariant: the factory that creates the rule and the
        // rule itself must share the same name.
        Ok(Box::new(FixNameRule::new(rule, self.new_name.clone())))
    }

    fn create_default(&self) -> Option<Box<dyn ErasedRule>> {
        self.inner.create_default()
    }
}

struct FixNameRule {
    inner: Box<dyn ErasedRule + Send + Sync + 'static>,
    new_name: String,
}

impl serde::Serialize for FixNameRule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl FixNameRule {
    pub fn new(
        inner: Box<dyn ErasedRule + Send + Sync + 'static>,
        new_name: impl Into<String>,
    ) -> Self {
        Self {
            inner,
            new_name: new_name.into(),
        }
    }
}

impl RuleMetadata for FixNameRule {
    fn name(&self) -> &str {
        &self.new_name
    }

    fn description(&self) -> &str {
        self.inner.description()
    }

    fn category(&self) -> Category {
        self.inner.category()
    }
}

impl ErasedRuleRunner for FixNameRule {
    fn check(
        &self,
        content: &crate::rule::Content,
        note: &crate::Note,
    ) -> Result<Vec<crate::rule::Violation>, Box<dyn std::error::Error>> {
        self.inner.check(content, note)
    }
}

#[must_use]
#[expect(clippy::missing_panics_doc, reason = "It is only test code")]
pub fn setup_registry(rule_count: usize) -> RuleFactoryRegistry {
    let mut registry = RuleFactoryRegistry::new();

    let fabrics: &[fn() -> Box<dyn ErasedRuleFactory + Send + Sync + 'static>] = &[
        || HeadingSpacingFactory.into_erased(),
        || DateFormatFactory.into_erased(),
        || ForbiddenWordsFactory.into_erased(),
        || EmptyFileFactory.into_erased(),
        || TagConsistencyFactory.into_erased(),
    ];

    for i in 0..rule_count {
        let idx = i % 5;
        let factory = fabrics[idx]();

        let fixed_factory = {
            let new_name = generate_name_by_index(i);

            let fixed = FixNameRuleFactory::new(factory, new_name);
            Box::new(fixed) as Box<dyn ErasedRuleFactory + Send + Sync + 'static>
        };

        #[expect(clippy::panic)]
        if let Some(prev_factory) = registry.add(fixed_factory) {
            panic!("Duplicate detected! Factory: `{}`", prev_factory.name());
        }
    }

    registry
}

/// Data for [`generate_data_for_test_rules`]
pub struct TestData {
    pub json: String,
    pub registry: RuleFactoryRegistry,
}

/// Get test data for testing deserialize [`Rules`] and [`RuleFabricRegistry`]
///
/// # Example
///
/// ```
/// use obsidian_tidy_core::bench_utils::{TestData, generate_data_for_test_rules};
/// use obsidian_tidy_core::rule::RulesSeed;
/// use serde::de::DeserializeSeed;
///
/// let TestData { json, registry } = generate_data_for_test_rules(20);
///
/// let seed = RulesSeed::new(&registry);
/// let rules = seed
///     .deserialize(&mut serde_json::Deserializer::from_str(&json))
///     .unwrap();
///
/// assert_eq!(rules.len(), 20);
/// ```
///
/// [`Rules`]: crate::rule::Rules
#[must_use]
pub fn generate_data_for_test_rules(rule_count: usize) -> TestData {
    let json = self::generate_benchmark_json(rule_count);
    let registry = self::setup_registry(rule_count);

    TestData { json, registry }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule::RulesSeed;
    use serde::de::DeserializeSeed;

    #[test]
    fn fix_name_factory() {
        let factory = DateFormatFactory;
        let name = factory.name().to_string();

        let fixed_factory = FixNameRuleFactory::new(factory.into_erased(), "fixed_name");
        let fixed_name = fixed_factory.name();

        assert_eq!(fixed_name, "fixed_name");
        assert_ne!(name, fixed_name);
    }

    #[test]
    fn fixed_factory_and_rule_have_same_name() {
        let factory = DateFormatFactory;
        let fixed_factory = FixNameRuleFactory::new(factory.into_erased(), "fixed_name");

        let mut deserializer = serde_json::Deserializer::from_str(r#"{"format": "YYYY-MM-DD"}"#);
        let mut deserializer = <dyn erased_serde::Deserializer>::erase(&mut deserializer);
        let fixed_rule = fixed_factory.create_by_serde(&mut deserializer).unwrap();

        assert_eq!(fixed_factory.name(), fixed_rule.name());
        assert_eq!(fixed_factory.name(), "fixed_name");
    }

    #[test]
    fn check_valid_generated_json() {
        let json = generate_benchmark_json(20);
        insta::assert_snapshot!(json);
    }

    #[test]
    fn check_setup_registry() {
        let registry = setup_registry(20);
        assert_eq!(registry.len(), 20);
    }

    #[test]
    fn get_rules_by_json() {
        let json = generate_benchmark_json(20);
        let registry = setup_registry(20);

        let seed = RulesSeed::new(&registry);
        let rules = seed
            .deserialize(&mut serde_json::Deserializer::from_str(&json))
            .unwrap();

        assert_eq!(rules.len(), 20);
    }

    #[test]
    fn all() {
        let TestData { json, registry } = generate_data_for_test_rules(20);

        let seed = RulesSeed::new(&registry);
        let rules = seed
            .deserialize(&mut serde_json::Deserializer::from_str(&json))
            .unwrap();

        assert_eq!(rules.len(), 20);
    }
}
