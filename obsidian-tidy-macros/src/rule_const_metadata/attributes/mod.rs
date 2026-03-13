//! Parsing of the `#[rule_metadata(...)]` attribute.

mod category;
mod rule_description;
mod rule_name;

use syn::{Error, Ident, LitStr, Token, parse::ParseStream};

pub use category::Category;
pub use rule_description::RuleDescription;
pub use rule_name::RuleName;

pub struct Attributes {
    pub name: RuleName,
    pub description: RuleDescription,
    pub category: Category,
}

impl syn::parse::Parse for Attributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut name = None;
        let mut description = None;
        let mut category = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            let _: Token![=] = input.parse()?;

            if key == "name" {
                if name.is_some() {
                    return Err(Error::new(key.span(), "Duplicate `name` attribute"));
                }

                let lit: LitStr = input.parse()?;
                name = Some(RuleName::new(lit.value(), lit.span())?);
            } else if key == "description" {
                if description.is_some() {
                    return Err(Error::new(key.span(), "Duplicate `description` attribute"));
                }

                let lit: LitStr = input.parse()?;
                description = Some(RuleDescription::new(lit.value(), lit.span())?);
            } else if key == "category" {
                if category.is_some() {
                    return Err(Error::new(key.span(), "Duplicate `category` attribute"));
                }

                let lit: Category = input.parse()?;
                category = Some(lit);
            } else {
                return Err(Error::new(key.span(), format!("Unknown attribute `{key}`")));
            }

            // Если после пары есть запятая, пропускаем её
            if input.peek(Token![,]) {
                let _: Token![,] = input.parse()?;
            } else {
                break;
            }
        }

        let name =
            name.ok_or_else(|| Error::new(input.span(), "Missing required attribute `name`"))?;

        let description = description
            .ok_or_else(|| Error::new(input.span(), "Missing required attribute `description`"))?;
        let category = category
            .ok_or_else(|| Error::new(input.span(), "Missing required attribute `category`"))?;

        Ok(Self {
            name,
            description,
            category,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rule_const_metadata::attributes::category::CoreCategory;
    use syn::parse_quote;

    #[test]
    fn parse_valid_attributes() {
        let attrs: Attributes = parse_quote! {
            name = "valid-rule",
            description = "This rule does something useful",
            category = Category::Content
        };

        assert_eq!(&*attrs.name, "valid-rule");
        assert_eq!(&*attrs.description, "This rule does something useful");
        assert_eq!(*attrs.category, CoreCategory::Content);
    }

    #[test]
    #[should_panic = "Duplicate `description` attribute"]
    fn duplicate_description() {
        let _: Attributes = parse_quote! {
            name = "valid-rule",
            description = "This rule does something useful",
            description = "Bug",
            category = Category::Content
        };
    }

    #[test]
    #[should_panic = "Duplicate `category` attribute"]
    fn duplicate_category() {
        let _: Attributes = parse_quote! {
            name = "valid-rule",
            description = "This rule does something useful",
            category = Category::Content,
            category = Category::Content
        };
    }

    #[test]
    #[should_panic = "Unknown attribute `invalid`"]
    fn unknown_attribute() {
        let _: Attributes = parse_quote! {
            name = "valid-rule",
            description = "desc",
            invalid = "value",
            category = Category::Content
        };
    }
}
