use crate::rule::ErasedRuleFactory;

pub use inventory as _inventory;

pub type RuleFactoryInventory = &'static (dyn ErasedRuleFactory + Send + Sync);
inventory::collect!(RuleFactoryInventory);

pub fn get_all_rule_factories() -> impl Iterator<Item = &'static RuleFactoryInventory> {
    inventory::iter()
}

#[macro_export]
macro_rules! registration_rule_factory {
    ($factory:ident) => {
        $crate::rule::factory::inventory::_inventory::submit! {
            &$factory as $crate::prelude::RuleFactoryInventory
        }
    };
}
