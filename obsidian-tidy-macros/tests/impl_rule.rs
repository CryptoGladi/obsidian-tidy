use obsidian_tidy_core::prelude::*;
use obsidian_tidy_macros::Rule;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

#[derive(Rule, Serialize, Deserialize)]
#[rule_metadata(
    name = "my-rule",
    description = "My rule",
    category = Category::Content,
    default
)]
struct MyRule {
    violation_message: String,
}

impl Default for MyRule {
    fn default() -> Self {
        Self {
            violation_message: "super string".to_string(),
        }
    }
}

impl RuleRunner for MyRule {
    type Error = Infallible;

    fn check(&self, _content: &Content, _note: &Note) -> Result<Vec<Violation>, Self::Error> {
        let violation = Violation::new(self.violation_message.clone(), ..);
        Ok(vec![violation])
    }
}

fn assert_my_rule_metadata(rule: &dyn ErasedRule) {
    assert_eq!(rule.name(), "my-rule");
    assert_eq!(rule.description(), "My rule");
    assert_eq!(rule.category(), Category::Content);
    assert_eq!(
        rule.check(&Content::default(), &Note::default()).unwrap()[0].message(),
        "super string"
    );
}

#[test]
fn check_metadata() {
    let my_rule = MyRule::default();

    assert_my_rule_metadata(&my_rule);
}

#[test]
fn check_registry() {
    let registry = RuleFactoryRegistry::new_by_inventory();
    assert_eq!(registry.len(), 1);

    let factory = registry.get("my-rule").unwrap();
    let my_rule = factory.create_default().unwrap();

    assert_my_rule_metadata(my_rule.as_ref());
}
