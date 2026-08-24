macro_rules! impl_enum_tag {
    (@match_arm Self::$variant:ident ( $internal_type:ty )) => {
        Self::$variant(_)
    };
    (@match_arm Self::$variant:ident) => {
        Self::$variant
    };

    // Main macro function
    {
        $( #[$meta_main:meta] )*
        $vis_main:vis enum $name_main:ident $( <$a:lifetime> )? {
            $(
                $( #[$meta_variant:meta] )*
                $variant:ident $( ( $internal_type:ty ) )?
            ),*

            $(,)?
        }

        $( #[$meta_end:meta] )*
        $vis_end:vis enum $name_end:ident { ... }
    } => {
        ::pastey::paste! {
            $( #[$meta_main] )*
            #[derive(::derive_more::IsVariant, ::strum::Display)]
            $vis_main enum $name_main<$( $a )?> {
                $(
                    $( #[$meta_variant] )*
                    $variant $( ( $internal_type ) )?,
                )*
            }

            #[doc = concat!("End version for [`", stringify!($name_main), "`].")]
            $( #[$meta_end] )*
            #[derive(::derive_more::IsVariant, ::strum::Display)]
            $vis_end enum $name_end {
                $(
                    #[doc = concat!(
                        "End version for [`",
                        stringify!($name_main),
                        "::",
                        stringify!($variant),
                        "`]."
                    )]
                    $( #[$meta_variant] )*
                    $variant,
                )*
            }

            #[allow(dead_code)]
            impl<$( $a )?> $name_main<$( $a )?> {
                $vis_main const fn to_end(&self) -> $name_end {
                    match self {
                        $(
                            impl_enum_tag!(@match_arm Self::$variant $( ($internal_type) )? ) => {
                                $name_end::$variant
                            }
                        )*
                    }
                }

                $(
                    $(
                        #[must_use]
                        $vis_main const fn [<as_ $variant:snake>](&self) -> ::core::option::Option<&$internal_type> {
                            if let Self::$variant(data) = self {
                                return ::core::option::Option::Some(data);
                            }

                            ::core::option::Option::None
                        }

                        #[must_use]
                        $vis_main fn [<into_ $variant:snake>](self) -> ::core::option::Option<$internal_type> {
                            if let Self::$variant(data) = self {
                                return ::core::option::Option::Some(data);
                            }

                            ::core::option::Option::None
                        }
                    )?
                )*
            }

            impl<'block, $( $a )?> From<&'block $name_main<$( $a )?>> for $name_end {
                fn from(tag: &$name_main<$( $a )?>) -> $name_end {
                    tag.to_end()
                }
            }
        }
    };
}

pub(super) use impl_enum_tag;

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

    impl_enum_tag! {
        #[allow(dead_code)]
        #[derive(Debug, Clone, PartialEq, Serialize)]
        pub enum MyTag<'input> {
            Paragraph,
            Heading(Heading),
            CodeBlock(CodeBlock<'input>),
            BlockQuote,
            Callout(Callout<'input>),
            Strong,
            Emphasis,
            HtmlBlock,
            List(List),
            Item,
        }

        /// Doc for [`TagEnd`]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum MyTagEnd { ... }
    }

    static_assertions::assert_impl_all!(MyTag: serde::Serialize);
    static_assertions::assert_not_impl_all!(MyTagEnd: serde::Serialize);

    static_assertions::assert_not_impl_all!(MyTag: std::hash::Hash);
    static_assertions::assert_impl_all!(MyTagEnd: std::hash::Hash);

    #[test]
    fn test_to_end_conversion() {
        assert_eq!(MyTag::Paragraph.to_end(), MyTagEnd::Paragraph);
        assert_eq!(MyTag::Item.to_end(), MyTagEnd::Item);

        // Проверяем варианты со сложными структурами и лайфтаймами
        let code_tag = MyTag::CodeBlock(CodeBlock("fn main() {}"));
        assert_eq!(code_tag.to_end(), MyTagEnd::CodeBlock);

        let callout_tag = MyTag::Callout(Callout("Warning!"));
        assert_eq!(callout_tag.to_end(), MyTagEnd::Callout);
    }

    #[test]
    fn test_from_trait_implementation() {
        let heading_tag = MyTag::Heading(Heading);

        let end_version = MyTagEnd::from(&heading_tag);

        assert_eq!(end_version, MyTagEnd::Heading);
        assert!(heading_tag.is_heading());
    }

    #[test]
    fn test_auto_generated_as_methods() {
        let text = "println!(\"Hello\");";
        let tag = MyTag::CodeBlock(CodeBlock(text));

        let code_ref = tag.as_code_block();
        assert!(code_ref.is_some());
        assert_eq!(code_ref.unwrap(), &CodeBlock(text));

        assert!(tag.as_heading().is_none());
    }

    #[test]
    fn auto_generated_into_methods() {
        let text = "println!(\"Hello\");";
        let tag = MyTag::CodeBlock(CodeBlock(text));
        let cloned_tag = tag.clone();

        let code_ref = tag.into_code_block();
        assert!(code_ref.is_some());
        assert_eq!(code_ref.unwrap(), CodeBlock(text));

        assert!(cloned_tag.as_heading().is_none());
    }

    #[test]
    fn test_derive_more_integration() {
        let tag = MyTag::BlockQuote;

        assert!(tag.is_block_quote());
        assert!(!tag.is_paragraph());

        let end_tag = MyTagEnd::HtmlBlock;
        assert!(end_tag.is_html_block());
    }

    #[test]
    fn test_copy_trait_on_tag_end() {
        let end_tag = MyTagEnd::Emphasis;
        let copied_tag = end_tag;

        assert_eq!(end_tag, copied_tag);
    }
}
