macro_rules! impl_enum_token {
    {
        $( #[$meta_enum:meta] )*
        $vis:vis enum $name:ident $( <$a:lifetime> )? {
            $(
                $( #[$meta_variant:meta] )*
                $variant:ident $( ( $internal_type:ty ) )?
            ),*

            $(,)?
        }
    } => {
        ::pastey::paste! {
            $( #[$meta_enum] )*
            #[derive(::derive_more::IsVariant, ::strum::Display)]
            $vis enum $name<$( $a )?> {
                $(
                    $( #[$meta_variant] )*
                    $variant $( ( $internal_type ) )?,
                )*
            }

            #[allow(dead_code, reason = "It is macros code")]
            impl<$( $a )?> $name<$( $a )?> {
                $(
                    $(
                        #[must_use]
                        $vis const fn [<as_ $variant:snake>](&self) -> ::core::option::Option<&$internal_type> {
                            if let Self::$variant(data) = self {
                                return ::core::option::Option::Some(data);
                            }

                            ::core::option::Option::None
                        }

                        #[must_use]
                        $vis fn [<into_ $variant:snake>](self) -> ::core::option::Option<$internal_type> {
                            if let Self::$variant(data) = self {
                                return ::core::option::Option::Some(data);
                            }

                            ::core::option::Option::None
                        }
                    )?
                )*
            }
        }
    };
}

pub(super) use impl_enum_token;

#[cfg(test)]
mod tests {
    use serde::Serialize;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub struct Heading;

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub struct CodeBlock<'a>(&'a str);

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub struct Callout<'a>(&'a str);

    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub struct List;

    impl_enum_token! {
        #[allow(dead_code)]
        #[derive(Debug, Clone, PartialEq, Serialize)]
        pub enum MyToken<'input> {
            Paragraph,
            Heading(Heading),
            Emphasis,
            CodeBlock(CodeBlock<'input>),
            Callout(Callout<'input>),
            HtmlBlock,
            List(List),
            Item,
        }
    }

    static_assertions::assert_impl_all!(MyToken: serde::Serialize);
    static_assertions::assert_not_impl_all!(MyToken: std::hash::Hash);

    #[test]
    fn auto_generated_as_methods() {
        let text = "println!(\"Hello\");";
        let tag = MyToken::CodeBlock(CodeBlock(text));

        let code_ref = tag.as_code_block();
        assert!(code_ref.is_some());
        assert_eq!(code_ref.unwrap(), &CodeBlock(text));

        assert!(tag.as_heading().is_none());
    }

    #[test]
    fn auto_generated_into_methods() {
        let text = "println!(\"Hello\");";
        let tag = MyToken::CodeBlock(CodeBlock(text));
        let cloned_tag = tag.clone();

        let code_ref = tag.into_code_block();
        assert!(code_ref.is_some());
        assert_eq!(code_ref.unwrap(), CodeBlock(text));

        assert!(cloned_tag.as_heading().is_none());
    }

    #[test]
    fn derive_more_integration() {
        let tag = MyToken::HtmlBlock;

        assert!(tag.is_html_block());
        assert!(!tag.is_paragraph());
    }
}
