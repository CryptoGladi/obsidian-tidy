//! Type for the `name` field of `#[rule_metadata]`.
//!
//! [`RuleName`] is a newtype around `String` that enforces validation rules:
//! - Must not be empty (even after trimming)
//! - Must contain only ASCII characters
//! - Must be in kebab-case (e.g., `"my-rule-name"`)
//! - Must not exceed 30 characters in length
//!
//! Validation is performed using a chain of responsibility (see [`crate::rule_const_metadata::chain_of_responsibility`]).
//! Compile-time errors are generated if any rule is violated.

use crate::rule_const_metadata::chain_of_responsibility::{
    CheckEmptyString, CheckKebabCase, CheckLenString, CheckOnlyAscii, handlers::Handlers, run_chain,
};
use proc_macro2::{Span, TokenStream};

use quote::TokenStreamExt;
use std::ops::Deref;
use syn::Error;

const MAX_LEN: usize = 30;

pub struct RuleName(String);

impl RuleName {
    pub fn new(str: String, span: Span) -> syn::Result<Self> {
        const EMPTY_MSG: &str =
            "Rule name cannot be empty string. Provide a non-empty value like name = \"my-rule\"";

        const ASCII_MSG: &str = "Rule name must contain only ASCII characters";
        const KEBAB_MSG: &str = "Rule name must be in kebab-case";

        #[allow(clippy::bytes_count_to_len, reason = "To support Unicode strings")]
        let long_msg = format!(
            "Rule name is very long\nThe maximum number of characters for a rule name is {}, and you have {}",
            MAX_LEN,
            str.bytes().count()
        );

        let handler = Handlers::new()
            .add(CheckEmptyString::new(EMPTY_MSG))
            .add(CheckOnlyAscii::new(ASCII_MSG))
            .add(CheckKebabCase::new(KEBAB_MSG))
            .add(CheckLenString::new(long_msg, MAX_LEN))
            .build_chain()
            .ok_or(Error::new(span, "No handlers configured"))?;

        run_chain(handler.as_ref(), &str, span)?;

        Ok(Self(str))
    }
}

impl Deref for RuleName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl syn::parse::Parse for RuleName {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lit: syn::LitStr = input.parse()?;
        let span = lit.span();
        let value = lit.value();

        RuleName::new(value, span)
    }
}

impl quote::ToTokens for RuleName {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let literal = proc_macro2::Literal::string(&self.0);
        tokens.append(literal);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal() {
        let _name = RuleName::new("cool-name".to_string(), Span::call_site()).unwrap();
    }

    #[test]
    #[should_panic]
    fn not_ascii() {
        let _name = RuleName::new("крутое-имя".to_string(), Span::call_site()).unwrap();
    }

    #[test]
    #[should_panic]
    fn empty() {
        let _name = RuleName::new("".to_string(), Span::call_site()).unwrap();
    }

    #[test]
    #[should_panic]
    fn whitespace() {
        let _name = RuleName::new("   ".to_string(), Span::call_site()).unwrap();
    }

    #[test]
    #[should_panic]
    fn not_kebab_case() {
        let _name = RuleName::new("cool_name".to_string(), Span::call_site()).unwrap();
    }

    #[test]
    #[should_panic]
    fn very_long() {
        let long_string = std::iter::repeat("a").take(100).collect();
        let _name = RuleName::new(long_string, Span::call_site()).unwrap();
    }
}
