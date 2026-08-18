mod callout;
mod code_block;
mod heading;
mod list;

pub use callout::{Callout, CalloutFoldable};
pub use code_block::CodeBlock;
pub use heading::{Heading, HeadingLevel};
pub use list::List;
use serde::Serialize;

macro_rules! impl_enum_tag {
    (@match_arm Self::$variant:ident ( $internal_type:ty )) => {
        Self::$variant(_)
    };
    (@match_arm Self::$variant:ident) => {
        Self::$variant
    };

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
            #[non_exhaustive]
            $vis enum $name<$( $a )?> {
                $(
                    $( #[$meta_variant] )*
                    $variant $( ( $internal_type ) )?,
                )*
            }

            #[doc = concat!("End version for [`", stringify!($name), "`].")]
            $( #[$meta_enum] )*
            #[derive(Copy, ::derive_more::IsVariant, ::strum::Display)]
            #[non_exhaustive]
            $vis enum [< $name End >] {
                $(
                    $( #[$meta_variant] )*
                    $variant,
                )*
            }

            impl<$( $a )?> $name<$( $a )?> {
                $vis const fn to_end(&self) -> [< $name End >] {
                    match self {
                        $(
                            impl_enum_tag!(@match_arm Self::$variant $( ($internal_type) )? ) => {
                                [< $name End >]::$variant
                            }
                        )*
                    }
                }
            }

            impl<'block, $( $a )?> From<&'block $name<$( $a )?>> for [<$name End>] {
                fn from(tag: &$name<$( $a )?>) -> [<$name End>] {
                    tag.to_end()
                }
            }
        }
    };
}

impl_enum_tag! {
    /// Tag for Start
    #[derive(Debug, Clone, PartialEq, Eq, Serialize)]
    pub enum Tag<'input> {
        Paragraph,

        Heading(Heading),
        CodeBlock(CodeBlock<'input>),
        BlockQuote,
        Callout(Callout<'input>),
        Strong,
        Emphasis,
        HtmlBlock,

        List(List),
        Item
    }

    // And TagEnd
}

// TagEnd it is simple enum
static_assertions::assert_impl_all!(TagEnd: Copy);
