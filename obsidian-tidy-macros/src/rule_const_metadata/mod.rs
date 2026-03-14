//! Implementation of the `RuleConstMetadata` derive macro.

pub mod attributes;
pub mod chain_of_responsibility;
pub mod kebab_case;

use attributes::Attributes;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Error};

pub fn rule_const_metadata_impl(input: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &input.ident;

    let rule_metadata = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("rule_metadata"))
        .ok_or(Error::new(
            name.span(),
            "Attribute #[rule_metadata(...)] is required",
        ))?;

    let Attributes {
        name: rule_name,
        description: rule_description,
        category: rule_category,
    } = rule_metadata.parse_args()?;

    let result = quote! {
        impl obsidian_tidy_core::rule::RuleConstMetadata for #name {
            const NAME: &'static str = #rule_name;
            const DESCRIPTION: &'static str = #rule_description;
            const CATEGORY: obsidian_tidy_core::rule::Category = #rule_category;
        }
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn valid() {
        let input: DeriveInput = parse_quote! {
            #[rule_metadata(
                name = "test-rule",
                description = "A test rule",
                category = Category::Other
            )]
            struct TestRule;
        };

        let result = rule_const_metadata_impl(&input).unwrap();
        let expected = quote! {
            impl obsidian_tidy_core::rule::RuleConstMetadata for TestRule {
                const NAME: &'static str = "test-rule";
                const DESCRIPTION: &'static str = "A test rule";
                const CATEGORY: obsidian_tidy_core::rule::Category = obsidian_tidy_core::rule::Category::Other;
            }
        };

        assert_eq!(result.to_string(), expected.to_string());
    }

    #[test]
    fn not_rule_metadata() {
        let input: DeriveInput = parse_quote! {
            struct TestRule;
        };

        let result = rule_const_metadata_impl(&input).unwrap_err();
        assert!(
            result
                .to_string()
                .contains("Attribute #[rule_metadata(...)] is required")
        );
    }

    // There is no point in doing more testing, since the other components have already been tested
}
