use obsidian_tidy_core::prelude::*;
use obsidian_tidy_macros::Rule;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

#[derive(Rule, Serialize, Deserialize)]
#[rule_metadata(
    name = "without-default",
    description = "Rule without `Default:default` implementation",
    category = Category::Content,
)]
struct WithoutDefault;

impl RuleRunner for WithoutDefault {
    type Error = Infallible;

    fn check(&self, _content: &Content, _note: &Note) -> Result<Vec<Violation>, Self::Error> {
        Ok(vec![])
    }
}

fn assert_my_rule_metadata(rule: &dyn ErasedRule) {
    assert_eq!(rule.name(), "without-default");
    assert_eq!(
        rule.description(),
        "Rule without `Default:default` implementation"
    );
    assert_eq!(rule.category(), Category::Content);

    assert!(
        rule.check(&Content::default(), &Note::default())
            .unwrap()
            .is_empty()
    );
}

#[test]
fn check_metadata() {
    let my_rule = WithoutDefault;

    assert_my_rule_metadata(&my_rule);
}

#[test]
fn check_registry() {
    let registry = RuleFactoryRegistry::new_by_inventory();
    assert_eq!(registry.len(), 1);

    let factory = registry.get("without-default").unwrap();
    assert!(factory.create_default().is_none());
}
