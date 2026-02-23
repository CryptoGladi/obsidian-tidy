use super::Rules;
use crate::rule::{Category, Rule, ToggleableRule};
use ::serde::{Deserialize, Serialize, Serializer};
use serde::{Deserializer, de::DeserializeSeed};
use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};
use tracing::{instrument, trace};

#[derive(Debug, Serialize, Deserialize)]
pub struct RuleConfig {
    pub enable: bool,
}

type RuleName = String;
type CategoryRules = BTreeMap<RuleName, RuleConfig>;

/// Impl Ls for serde
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct InnerRules(BTreeMap<Category, CategoryRules>);

impl InnerRules {
    #[instrument]
    fn add_rule(&mut self, name: RuleName, category: Category, enable: bool) {
        trace!("Add rule");

        self.entry(category)
            .or_default()
            .insert(name, RuleConfig { enable });
    }
}

impl Deref for InnerRules {
    type Target = BTreeMap<Category, CategoryRules>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InnerRules {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<R> Serialize for Rules<R>
where
    R: Rule,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut rules = InnerRules::default();

        for rule in &self.0 {
            rules.add_rule(rule.name().to_string(), rule.category(), rule.is_enabled());
        }

        serializer.serialize_newtype_struct("rules", &rules)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RulesSeed<'a, R>
where
    R: Rule,
{
    available_rules: &'a Vec<R>,
}

impl<'a, R> RulesSeed<'a, R>
where
    R: Rule,
{
    #[must_use]
    pub const fn new(available_rules: &'a Vec<R>) -> Self {
        Self { available_rules }
    }
}

impl<'de, R> DeserializeSeed<'de> for RulesSeed<'_, R>
where
    R: Rule + Clone,
{
    type Value = Rules<R>;

    #[instrument(skip(self, deserializer), err)]
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        trace!("Run deserialize");

        let inner = InnerRules::deserialize(deserializer)?;
        let mut vec_rules = Vec::with_capacity(self.available_rules.len());

        for map in inner.0.values() {
            for (name, config) in map {
                let rule = self
                    .available_rules
                    .iter()
                    .find(|rule| rule.name() == name)
                    .ok_or(serde::de::Error::missing_field(
                        "not found rule from `available_rules`",
                    ))?;

                vec_rules.push(ToggleableRule::new(rule.clone(), config.enable));
            }
        }

        Rules::new(vec_rules).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
#[allow(clippy::similar_names)]
mod tests {
    use super::*;
    use crate::rule::ToggleableRule;
    use crate::test_utils::TestRule;
    use std::sync::Arc;

    #[test]
    fn serialize() {
        let rule1 = ToggleableRule::new(
            Arc::new(TestRule::new("rule1", "", Category::Content, [])),
            true,
        );
        let rule2 = ToggleableRule::new(
            Arc::new(TestRule::new("rule2", "", Category::Spacing, [])),
            false,
        );

        let rules = Rules::new(vec![rule1, rule2]).unwrap();
        let toml = toml::to_string(&rules).unwrap();

        assert_eq!(
            toml,
            r"[content.rule1]
enable = true

[spacing.rule2]
enable = false
"
        );
    }

    #[test]
    fn deserialize() {
        let rule1 = Arc::new(TestRule::new("rule1", "", Category::Content, []));
        let rule2 = Arc::new(TestRule::new("rule2", "", Category::Spacing, []));

        let toggleable_rule1 = ToggleableRule::new(rule1.clone(), true);
        let toggleable_rule2 = ToggleableRule::new(rule2.clone(), false);

        let rules = Rules::new(vec![toggleable_rule1, toggleable_rule2]).unwrap();
        let toml = toml::to_string(&rules).unwrap();

        let available_rules = vec![rule1, rule2];
        let rules_deserialized = RulesSeed::new(&available_rules)
            .deserialize(toml::Deserializer::parse(&toml).unwrap())
            .unwrap();

        assert_eq!(rules, rules_deserialized);
    }

    #[test]
    #[should_panic(expected = "not found rule")]
    fn deserialize_with_not_found_rule() {
        let rule1 = Arc::new(TestRule::new("rule1", "", Category::Content, []));
        let rule2 = Arc::new(TestRule::new("rule2", "", Category::Spacing, []));

        let toggleable_rule1 = ToggleableRule::new(rule1.clone(), true);
        let toggleable_rule2 = ToggleableRule::new(rule2.clone(), false);

        let rules = Rules::new(vec![toggleable_rule1, toggleable_rule2]).unwrap();
        let toml = toml::to_string(&rules).unwrap();

        let available_rules = vec![rule1]; // Without rule2
        let rules_deserialized = RulesSeed::new(&available_rules)
            .deserialize(toml::Deserializer::parse(&toml).unwrap())
            .unwrap();

        assert_eq!(rules, rules_deserialized);
    }
}
