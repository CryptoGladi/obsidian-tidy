//! Procedural macros for the obsidian-tidy

#![forbid(clippy::print_stdout)]
#![forbid(clippy::print_stderr)]

pub(crate) mod rule_const_metadata;

#[cfg(test)]
pub(crate) mod test_utils;

use proc_macro::TokenStream;

/// This crate provides a custom derive macro [`RuleConstMetadata`] that automatically
/// implements the [`obsidian_tidy_core::rule::RuleConstMetadata`] trait for rule structs.
/// This macro simplifies the definition of lint rules by generating constant metadata
/// from declarative attributes.
///
/// # Usage
///
/// Add this crate to your `Cargo.toml`:
///
/// ```toml
/// [dependencies]
/// obsidian-tidy-macros.workspace = true
/// obsidian-tidy-core.workspace = true
/// ```
///
/// Then import the macro and derive it on your rule struct:
///
/// ```
/// use obsidian_tidy_macros::RuleConstMetadata;
///
/// #[derive(RuleConstMetadata)]
/// #[rule_metadata(
///     name = "no-empty-links",
///     description = "Detects empty markdown links",
///     category = Category::Content
/// )]
/// struct NoEmptyLinks;
/// ```
///
/// This will generate the following implementation:
///
/// ```
/// # use obsidian_tidy_core::rule::Category;
/// # struct NoEmptyLinks;
/// impl obsidian_tidy_core::rule::RuleConstMetadata for NoEmptyLinks {
///     const NAME: &'static str = "no-empty-links";
///     const DESCRIPTION: &'static str = "Detects empty markdown links";
///     const CATEGORY: obsidian_tidy_core::rule::Category = Category::Content;
/// }
/// ```
///
/// # Macro Attributes
///
/// The `#[rule_metadata(...)]` attribute accepts three **required** fields:
///
/// - `name`: A string literal in **kebab-case** (e.g., `"no-empty-links"`).
///   Must be non-empty, ASCII-only, and at most 30 characters long.
/// - `description`: A string literal describing the rule.
///   Must be non-empty, ASCII-only, and at most 65 characters long.
/// - `category`: One of the variants of [`obsidian_tidy_core::rule::Category`]:
///   `Content`, `Heading`, `Spacing`, `Yaml`, or `Other`.
///
/// # Validation
///
/// The macro performs validation at compile time. If any of the constraints are violated,
/// a descriptive error message will be emitted pointing to the exact location.
#[proc_macro_derive(RuleConstMetadata, attributes(rule_metadata))]
pub fn derive_rule(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    rule_const_metadata::rule_const_metadata_impl(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
