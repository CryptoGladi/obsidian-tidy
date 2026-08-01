use obsidian_tidy_core::rule::{RuleConstMetadata, RuleMetadata};
use obsidian_tidy_macros::RuleConstMetadata;

#[derive(Debug, Default, RuleConstMetadata)]
#[rule_metadata(
     name = "my-rule",
     description = "My rule",
     category = Category::Content
 )]
struct MyRule;

#[test]
fn my_rule() {
    let my_rule = MyRule::default();

    assert_eq!(my_rule.name(), "my-rule");
    assert_eq!(my_rule.name(), MyRule::NAME);

    assert_eq!(my_rule.description(), "My rule");
    assert_eq!(my_rule.description(), MyRule::DESCRIPTION);

    assert_eq!(my_rule.name(), MyRule::NAME);
    assert_eq!(my_rule.name(), MyRule::NAME);
}
