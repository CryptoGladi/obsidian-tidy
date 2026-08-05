//! Implementation of the `RuleConstMetadata` derive macro.

pub mod attributes;
pub mod chain_of_responsibility;
pub mod kebab_case;

use attributes::Attributes;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, Error};

pub fn rule_impl(input: &DeriveInput) -> syn::Result<TokenStream> {
    let ident = &input.ident;

    let rule_metadata = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("rule_metadata"))
        .ok_or_else(|| Error::new(ident.span(), "Attribute #[rule_metadata(...)] is required"))?;

    let Attributes {
        name: rule_name,
        description: rule_description,
        category: rule_category,
        default,
    } = rule_metadata.parse_args()?;

    let factory_ident = format_ident!("__{}_RuleFactory", ident);

    let impl_create_default = if default {
        quote! {
            const _: () = {
                const fn assert_traits<T: ::std::default::Default>() {}
                assert_traits::<#ident>();
            };

            Some(<#ident as ::std::default::Default>::default())
        }
    } else {
        quote! { None }
    };

    let result = quote! {
        impl ::obsidian_tidy_core::rule::RuleMetadata for #ident {
            fn name(&self) -> &'static str {
                #rule_name
            }

            fn description(&self) -> &'static str {
                #rule_description
            }

            fn category(&self) -> ::obsidian_tidy_core::rule::Category {
                #rule_category
            }
        }

        const _: () = {
            const fn assert_traits<T: ::serde::Serialize + ::serde::de::DeserializeOwned>() {}
            assert_traits::<#ident>();
        };

        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        struct #factory_ident;

        impl ::obsidian_tidy_core::rule::RuleFactory for #factory_ident {
            type Rule = #ident;
            type Data = #ident;
            type Error = ::std::convert::Infallible;

            fn id(&self) -> &'static str {
                #rule_name
            }

            fn create_by_serde(&self, data: Self::Data) -> Result<Self::Rule, Self::Error> {
                Ok(data)
            }

            fn create_default(&self) -> Option<Self::Rule> {
                #impl_create_default
            }
        }

        ::obsidian_tidy_core::registration_rule_factory!(#factory_ident);
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    //#[test]
    fn valid() {
        // TODO
        let input: DeriveInput = parse_quote! {
            #[rule_metadata(
                name = "test-rule",
                description = "A test rule",
                category = Category::Other
            )]
            struct TestRule;
        };

        let result = rule_impl(&input).unwrap();
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

        let result = rule_impl(&input).unwrap_err();
        assert!(
            result
                .to_string()
                .contains("Attribute #[rule_metadata(...)] is required")
        );
    }

    // There is no point in doing more testing, since the other components have already been tested
}
