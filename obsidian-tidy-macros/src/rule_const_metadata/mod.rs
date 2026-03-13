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
