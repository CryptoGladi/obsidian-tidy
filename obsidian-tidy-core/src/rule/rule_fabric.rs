use crate::rule::{Rule, SharedErrorRule};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::collections::HashMap;

pub trait RuleFabric {
    type Rule: super::Rule;

    fn create<'de, D>(&self, deserializer: D) -> Result<Self::Rule, D::Error>
    where
        D: serde::Deserializer<'de>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{rule::Category, test_utils::TestRule};

    #[derive(Debug, Default)]
    struct TestRuleFabric;

    impl RuleFabric for TestRuleFabric {
        type Rule = TestRule;

        fn create<'de, D>(&self, deserializer: D) -> Result<Self::Rule, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            TestRule::deserialize(deserializer)
        }
    }

    #[test]
    fn test_rule() {
        const TEST_NAME: &str = "Test name";
        const TEST_DESCRIPTION: &str = "test description";
        const TEST_CATEGORY: Category = Category::Heading;

        let test_rule = TestRule::new(TEST_NAME, TEST_DESCRIPTION, TEST_CATEGORY, []);
        let json = serde_json::to_string_pretty(&test_rule).unwrap();

        let mut deserializer = serde_json::Deserializer::from_str(&json);
        let deserialized_test_rule = TestRuleFabric::default().create(&mut deserializer).unwrap();

        assert_eq!(test_rule, deserialized_test_rule);
    }
}
