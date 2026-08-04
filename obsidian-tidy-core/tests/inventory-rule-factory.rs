use obsidian_tidy_core::{
    Note,
    rule::{
        Category, Content, RuleFactory, RuleMetadata, RuleRunner, Violation,
        factory::inventory::get_all_rule_factories,
    },
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::Infallible};

#[derive(Debug, Deserialize, Serialize)]
pub struct TestRule {
    format: String,
}

impl Default for TestRule {
    fn default() -> Self {
        Self {
            format: "YYYY-MM-DD".to_string(),
        }
    }
}

impl RuleMetadata for TestRule {
    fn name(&self) -> &'static str {
        "test"
    }

    fn description(&self) -> &'static str {
        "It is mock rule"
    }

    fn category(&self) -> Category {
        Category::Other
    }
}

impl RuleRunner for TestRule {
    type Error = Infallible;

    fn check(&self, _content: &Content, _note: &Note) -> Result<Vec<Violation>, Self::Error> {
        let violation = Violation::new(format!("Rule with `{}` format", self.format), ..);

        Ok(vec![violation])
    }
}

pub struct TestRuleFactory;

impl RuleFactory for TestRuleFactory {
    type Rule = TestRule;
    type Data = TestRule;
    type Error = Infallible;

    fn id(&self) -> &'static str {
        "test"
    }

    fn create_by_serde(&self, data: Self::Data) -> Result<Self::Rule, Self::Error> {
        Ok(data)
    }

    fn create_default(&self) -> Option<Self::Rule> {
        Some(Self::Rule::default())
    }
}

obsidian_tidy_core::registration_rule_factory!(TestRuleFactory);

#[test]
fn inventory_iter() {
    let rules = get_all_rule_factories()
        .map(|fabric| (fabric.name(), fabric.create_default().unwrap()))
        .collect::<HashMap<_, _>>();

    let result = rules
        .get("test")
        .unwrap()
        .check(&Content::default(), &Note::default())
        .unwrap();

    assert_eq!(result[0].message(), "Rule with `YYYY-MM-DD` format");
}
