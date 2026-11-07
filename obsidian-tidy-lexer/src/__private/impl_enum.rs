macro_rules! impl_enum {
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
                        $vis fn [<as_ $variant:snake>](&self) -> ::core::option::Option<&<$internal_type as $crate::__private::AsRefTarget>::Target>
                        where
                            $internal_type: $crate::__private::AsRefTarget,
                        {
                            if let Self::$variant(data) = self {
                                return ::core::option::Option::Some(
                                    $crate::__private::AsRefTarget::as_target_ref(data)
                                );
                            }

                            ::core::option::Option::None
                        }

                        #[must_use]
                        $vis fn [<as_mut_ $variant:snake>](&mut self) -> ::core::option::Option<&mut <$internal_type as $crate::__private::AsMutTarget>::Target>
                        where
                            $internal_type: $crate::__private::AsMutTarget,
                        {
                            if let Self::$variant(data) = self {
                                return ::core::option::Option::Some(
                                    $crate::__private::AsMutTarget::as_target_mut(data)
                                );
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

pub(crate) use impl_enum;

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

    crate::__private::impl_as_target_self!(Heading, CodeBlock<'_>, Callout<'_>, List);

    impl_enum! {
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
