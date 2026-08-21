mod adapter_pulldown_cmark;
pub mod tag;

pub use tag::*;

use alloc::borrow::Cow;
use derive_more::IsVariant;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, IsVariant, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Token<'input> {
    Start(Tag<'input>),
    End(TagEnd),
    Text(Cow<'input, str>),
    Code(Cow<'input, str>),
    SoftBreak,
    HardBreak,
    Rule,
}

macro_rules! impl_token_as_and_into {
    ($field:ident, $for_return:path) => {
        ::pastey::paste! {
            impl<'input> $crate::token_stream::token::Token<'input> {
                #[must_use]
                pub const fn [<as_ $field:snake>](&self) -> Option<&$for_return> {
                    if let $crate::token_stream::token::Token::$field(data) = self {
                        return Some(data);
                    }

                    None
                }

                #[must_use]
                pub fn [<into_ $field:snake>](self) -> Option<$for_return> {
                    if let $crate::token_stream::token::Token::$field(data) = self {
                        return Some(data);
                    }

                    None
                }
            }
        }
    };
}

impl_token_as_and_into!(Start, Tag<'input>);
impl_token_as_and_into!(End, TagEnd);

// TODO fix to &str
impl_token_as_and_into!(Text, Cow<'input, str>);
impl_token_as_and_into!(Code, Cow<'input, str>);

impl Token<'_> {
    #[must_use]
    #[inline]
    pub const fn is_break(&self) -> bool {
        self.is_hard_break() || self.is_soft_break()
    }
}
