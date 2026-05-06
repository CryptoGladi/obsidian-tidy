use obsidian_tidy_core::{
    rule::{
        Category, RuleFabricRegistry, Rules, ToggleableRule,
        rule_fabric::GetFabricFromRuleConstMetadata,
    },
    rule_fabric_registry, rules,
    test_utils::{ErrorRule, TestRule, TestRuleFabric},
};

pub fn test_rules() -> Rules {
    let rule1 = TestRule::default();
    let rule1 = ToggleableRule::new(rule1, true);

    let rule2 = TestRule::new("rule2", "Rule two", Category::Spacing, []);
    let rule2 = ToggleableRule::new(rule2, false);

    let rule3 = ErrorRule::default();
    let rule3 = ToggleableRule::new(rule3, false);

    rules![rule1, rule2, rule3]
}

pub fn test_registry_fabric() -> RuleFabricRegistry {
    let rule1 = TestRuleFabric::default();
    let rule2 = TestRuleFabric::new("rule2", "Rule two", Category::Spacing);
    let rule3 = ErrorRule::fabric();

    rule_fabric_registry![rule1, rule2, rule3]
}
