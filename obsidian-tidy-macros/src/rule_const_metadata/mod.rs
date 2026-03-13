mod category;
mod chain_of_responsibility;
mod kebab_case;
mod rule_description;
mod rule_name;

use crate::rule_const_metadata::{rule_description::RuleDescription, rule_name::RuleName};
use category::Category;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn rule_const_metadata_impl(input: DeriveInput) -> syn::Result<TokenStream> {
    let name = input.ident;

    let rule_metadata = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("rule_metadata"))
        .expect("Attribute #[rule_metadata(...)] is required");

    let mut rule_name = None;
    let mut rule_description = None;
    let mut rule_category = None;
    rule_metadata.parse_nested_meta(|meta| {
        if meta.path.is_ident("name") {
            let value = meta.value()?;
            let lit: RuleName = value.parse()?;
            rule_name = Some(lit);

            return Ok(());
        }

        if meta.path.is_ident("description") {
            let value = meta.value()?;
            let lit: RuleDescription = value.parse()?;
            rule_description = Some(lit);

            return Ok(());
        }

        if meta.path.is_ident("category") {
            let value = meta.value()?;
            let category: Category = value.parse()?;
            rule_category = Some(category);

            return Ok(());
        }

        Err(meta.error("Unrecognized rule_metadata"))
    })?;

    let rule_name = rule_name.expect("Attribute #[rule_metadata(name = \"...\")] is required");
    let rule_description =
        rule_description.expect("Attribute #[rule_metadata(description = \"...\")] is required");
    let rule_category =
        rule_category.expect("Attribute #[rule_metadata(category = ...)] is required");

    let result = quote! {
        impl obsidian_tidy_core::rule::RuleConstMetadata for #name {
            const NAME: &'static str = #rule_name;
            const DESCRIPTION: &'static str = #rule_description;
            const CATEGORY: obsidian_tidy_core::rule::Category = #rule_category;
        }
    };

    Ok(result.into())
}
