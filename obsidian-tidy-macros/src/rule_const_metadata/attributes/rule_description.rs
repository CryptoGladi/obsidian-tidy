//! Type for the `description` field of `#[rule_metadata]`.
//!
//! [`RuleDescription`] is a newtype around `String` that enforces validation rules:
//! - Must not be empty (even after trimming)
//! - Must contain only ASCII characters
//! - Must not exceed 65 characters in length
//!
//! Validation is performed using a chain of responsibility (see [`crate::rule_const_metadata::chain_of_responsibility`]).
//! Compile-time errors are generated if any rule is violated.

use crate::rule_const_metadata::chain_of_responsibility::{
    CheckEmptyString, CheckLenString, CheckOnlyAscii, handlers::Handlers, run_chain,
};
use proc_macro2::{Span, TokenStream};
use quote::TokenStreamExt;
use std::ops::Deref;
use syn::Error;

const MAX_LEN: usize = 65;

pub struct RuleDescription(String);

impl RuleDescription {
    pub fn new(str: String, span: Span) -> syn::Result<Self> {
        const EMPTY_MSG: &str = "Rule description cannot be empty string. Provide a non-empty value like name = \"My rule description\"";
        const ASCII_MSG: &str = "Rule description must contain only ASCII characters";

        #[allow(clippy::bytes_count_to_len, reason = "To support Unicode strings")]
        let long_msg = format!(
            "Rule description is very long\nThe maximum number of characters for a rule name is {}, and you have {}",
            MAX_LEN,
            str.bytes().count()
        );

        let handler = Handlers::new()
            .add(CheckEmptyString::new(EMPTY_MSG))
            .add(CheckOnlyAscii::new(ASCII_MSG))
            .add(CheckLenString::new(long_msg, MAX_LEN))
            .build_chain()
            .ok_or(Error::new(span, "No handlers configured"))?;

        run_chain(handler.as_ref(), &str, span)?;

        Ok(Self(str))
    }
}

impl Deref for RuleDescription {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl syn::parse::Parse for RuleDescription {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lit: syn::LitStr = input.parse()?;
        let span = lit.span();
        let value = lit.value();

        RuleDescription::new(value, span)
    }
}

impl quote::ToTokens for RuleDescription {
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
        let _ = RuleDescription::new("Cool description".to_string(), Span::call_site()).unwrap();
    }

    #[test]
    #[should_panic]
    fn not_ascii() {
        let _ = RuleDescription::new("Крутое описание".to_string(), Span::call_site()).unwrap();
    }

    #[test]
    #[should_panic]
    fn empty() {
        let _ = RuleDescription::new("".to_string(), Span::call_site()).unwrap();
    }

    #[test]
    #[should_panic]
    fn whitespace() {
        let _ = RuleDescription::new("   ".to_string(), Span::call_site()).unwrap();
    }

    #[test]
    #[should_panic]
    fn very_long() {
        let long_string = std::iter::repeat("A").take(100).collect();
        let _name = RuleDescription::new(long_string, Span::call_site()).unwrap();
    }
}
