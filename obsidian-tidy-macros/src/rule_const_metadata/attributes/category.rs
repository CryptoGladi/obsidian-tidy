//! Wrapper for the `Category` enum from `obsidian_tidy_core`.
//!
//! This module provides a newtype [`Category`]

pub use obsidian_tidy_core::rule::Category as CoreCategory;

use quote::{TokenStreamExt, quote};
use std::ops::Deref;

#[derive(Debug)]
pub struct Category(CoreCategory);

impl Deref for Category {
    type Target = CoreCategory;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl syn::parse::Parse for Category {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path: syn::Path = input.parse()?;
        let segments: Vec<_> = path.segments.iter().collect();

        if segments.len() != 2 {
            return Err(syn::Error::new_spanned(
                path,
                "Expected Category::Variant (e.g., Category::Yaml, Category::Heading)",
            ));
        }

        let category_segment = segments[0];
        if category_segment.ident != "Category" {
            return Err(syn::Error::new_spanned(
                category_segment,
                format!("Expected Category, found {}", category_segment.ident),
            ));
        }

        let variant_segment = segments[1];
        let variant_name = variant_segment.ident.to_string();

        match variant_name.as_str() {
            "Yaml" => Ok(Category(CoreCategory::Yaml)),
            "Heading" => Ok(Category(CoreCategory::Heading)),
            "Content" => Ok(Category(CoreCategory::Content)),
            "Spacing" => Ok(Category(CoreCategory::Spacing)),
            "Other" => Ok(Category(CoreCategory::Other)),
            _ => Err(syn::Error::new_spanned(
                variant_segment,
                format!(
                    "Unknown category variant '{variant_name}'. Expected one of: Yaml, Heading, Content, Spacing, Other"
                ),
            )),
        }
    }
}

impl quote::ToTokens for Category {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let variant = match self.0 {
            CoreCategory::Yaml => quote! { Yaml },
            CoreCategory::Heading => quote! { Heading },
            CoreCategory::Content => quote! { Content },
            CoreCategory::Spacing => quote! { Spacing },
            CoreCategory::Other => quote! { Other },
        };

        tokens.append_all(quote! { obsidian_tidy_core::rule::Category::#variant });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::ToTokens;
    use syn::{Error, parse_quote};

    #[test]
    fn parse() {
        let test_cases = vec![
            (quote! { Category::Yaml }, CoreCategory::Yaml),
            (quote! { Category::Heading }, CoreCategory::Heading),
            (quote! { Category::Content }, CoreCategory::Content),
            (quote! { Category::Spacing }, CoreCategory::Spacing),
            (quote! { Category::Other }, CoreCategory::Other),
        ];

        for (path, expected) in test_cases {
            let result: Category = syn::parse2(path).unwrap();
            assert_eq!(*result, expected);
        }
    }

    #[test]
    fn to_tokens() {
        let test_cases = vec![
            (
                CoreCategory::Yaml,
                quote! { obsidian_tidy_core::rule::Category::Yaml },
            ),
            (
                CoreCategory::Heading,
                quote! { obsidian_tidy_core::rule::Category::Heading },
            ),
            (
                CoreCategory::Content,
                quote! { obsidian_tidy_core::rule::Category::Content },
            ),
            (
                CoreCategory::Spacing,
                quote! { obsidian_tidy_core::rule::Category::Spacing },
            ),
            (
                CoreCategory::Other,
                quote! { obsidian_tidy_core::rule::Category::Other },
            ),
        ];

        for (core_category, expected_tokens) in test_cases {
            let category = Category(core_category);
            let tokens = category.into_token_stream();

            assert_eq!(tokens.to_string(), expected_tokens.to_string());
        }
    }

    #[test]
    fn test_invalid_category_variants() {
        let invalid_variants = vec!["None", "Invalid", "Style", "Format", "Unknown"];

        for variant in invalid_variants {
            let path: syn::Path = syn::parse_str(&format!("Category::{}", variant)).unwrap();
            let error: Error = syn::parse2::<Category>(path.into_token_stream()).unwrap_err();

            let error_string = error.to_string();

            assert!(error_string.contains(&format!("Unknown category variant '{}'", variant)));
            assert!(
                error_string.contains("Expected one of: Yaml, Heading, Content, Spacing, Other")
            );
        }
    }

    #[test]
    fn test_wrong_path_format() {
        let path: syn::Path = parse_quote! { Yaml };
        let error = syn::parse2::<Category>(path.into_token_stream()).unwrap_err();
        assert!(error.to_string().contains("Expected Category::Variant"));

        let path: syn::Path = parse_quote! { crate::Category::Yaml };
        let error = syn::parse2::<Category>(path.into_token_stream()).unwrap_err();
        assert!(error.to_string().contains("Expected Category::Variant"));

        let path: syn::Path = parse_quote! { OtherEnum::Yaml };
        let error = syn::parse2::<Category>(path.into_token_stream()).unwrap_err();
        assert!(error.to_string().contains("Expected Category, found"));
    }

    #[test]
    fn test_empty_path() {
        let result = syn::parse2::<Category>(quote! {});
        assert!(result.is_err());
    }
}
