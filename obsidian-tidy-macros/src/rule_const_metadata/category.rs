use obsidian_tidy_core::rule::Category as CoreCategory;
use quote::{TokenStreamExt, quote};
use std::ops::Deref;

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

        if let Some(segment) = path.segments.last() {
            match segment.ident.to_string().as_str() {
                "Content" => Ok(Category(CoreCategory::Content)),
                "Heading" => Ok(Category(CoreCategory::Heading)),
                "Spacing" => Ok(Category(CoreCategory::Spacing)),
                "Yaml" => Ok(Category(CoreCategory::Yaml)),
                "Other" => Ok(Category(CoreCategory::Other)),
                _ => Err(syn::Error::new_spanned(path, "Unknown category")),
            }
        } else {
            Err(syn::Error::new_spanned(path, "Invalid category format"))
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

        tokens.append_all(quote! { Category::#variant });
    }
}
