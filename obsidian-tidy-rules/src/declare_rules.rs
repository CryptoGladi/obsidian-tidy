macro_rules! declare_rules {
    (
        $(
            [$rule_type:ty, $fabric_expr:expr]
        ),+ $(,)?
    ) => {
        pub static ALL_RULES_FABRICS: std::sync::LazyLock<obsidian_tidy_core::rule::RuleFabricRegistry> =
            std::sync::LazyLock::new(|| {
                obsidian_tidy_core::rule_fabric_registry![
                    $($fabric_expr),*
                ]
            });

        pub fn create_all_default_rules() -> Vec<Box<dyn obsidian_tidy_core::rule::ErasedRule>> {
            use obsidian_tidy_core::rule::erased_rule::GetErasedRule;

            vec![
                $(<$rule_type>::default().into_erased()),*
            ]
        }
    };
}

pub(crate) use declare_rules;
