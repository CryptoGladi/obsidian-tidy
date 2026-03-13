pub(crate) mod rule_const_metadata;

use proc_macro::TokenStream;

/// Proc macro for generate RuleConstMetadata
///
/// # Example
/// ```
/// use obsidian_tidy_macros::RuleConstMetadata;
/// use obsidian_tidy_core::rule::Category;
///
/// #[derive(RuleConstMetadata)]
/// #[rule_metadata(
///     name = "my-rule",
///     description = "My rule description",
///     category = Category::Content
/// )]
/// struct MyRule;
/// ```
#[proc_macro_derive(RuleConstMetadata, attributes(rule_metadata))]
pub fn derive_rule(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    rule_const_metadata::rule_const_metadata_impl(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
