mod adapter_pulldown_cmark;
mod impl_enum_token;
mod tag;
mod task_list_marker;

pub use tag::*;
pub use task_list_marker::TaskListMarker;

use alloc::borrow::Cow;
use impl_enum_token::impl_enum_token;

impl_enum_token! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[non_exhaustive]
    pub enum Token<'input> {
        Start(Tag<'input>),
        End(TagEnd),

        Text(Cow<'input, str>),
        Code(Cow<'input, str>),
        FootnoteReference(Cow<'input, str>),

        SoftBreak,
        HardBreak,

        InlineMath(Cow<'input, str>),
        DisplayMath(Cow<'input, str>),

        Html(Cow<'input, str>),
        InlineHtml(Cow<'input, str>),

        Rule,
        TaskListMarker(TaskListMarker)
    }
}

impl<'input> Token<'input> {
    #[must_use]
    #[inline]
    pub const fn is_break(&self) -> bool {
        self.is_hard_break() || self.is_soft_break()
    }

    #[must_use]
    #[inline]
    pub const fn is_math(&self) -> bool {
        self.is_display_math() || self.is_inline_math()
    }

    #[must_use]
    pub fn as_math(&self) -> Option<&str> {
        self.as_display_math()
            .or(self.as_inline_math())
            .map(Cow::as_ref)
    }

    #[must_use]
    pub fn into_math(self) -> Option<Cow<'input, str>> {
        match self {
            Self::DisplayMath(text) | Self::InlineMath(text) => Some(text),
            _ => None,
        }
    }
}
