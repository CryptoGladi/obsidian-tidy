use crate::rule::{DynRule, Rule, SharedErrorRule, rule_fabric::RuleFabric};
use std::{any::Any, collections::HashMap};

struct ErasingRuleFabric<R: RuleFabric> {
    inner: R,
}

impl<R> ErasingRuleFabric<R>
where
    R: RuleFabric,
{
    fn new(rule_fabric: R) -> Self {
        Self { inner: rule_fabric }
    }
}

impl<R> RuleFabric for ErasingRuleFabric<R>
where
    R: RuleFabric,
    R::Rule: 'static,
    <R::Rule as Rule>::Error: Send + Sync + 'static,
{
    type Rule = SharedErrorRule;

    fn create<'de, D>(&self, deserializer: D) -> Result<Self::Rule, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let rule: R::Rule = self.inner.create(deserializer)?;
        Ok(SharedErrorRule::new(rule))
    }
}
