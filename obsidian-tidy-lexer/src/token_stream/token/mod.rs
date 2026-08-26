mod adapter_pulldown_cmark;
mod tag;
mod task_list_marker;

pub use tag::*;
pub use task_list_marker::TaskListMarker;

use crate::__private::impl_enum;
use alloc::borrow::Cow;

impl_enum! {
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
        self.as_display_math().or(self.as_inline_math())
    }

    #[must_use]
    pub fn into_math(self) -> Option<Cow<'input, str>> {
        match self {
            Self::DisplayMath(text) | Self::InlineMath(text) => Some(text),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{InterceptorEnum, Token, TokenStreamBuilder, TracingTokenStreamExt};
    use core::range::Range;

    fn token_stream(source: &str) -> Vec<(Token<'_>, Range<usize>)> {
        TokenStreamBuilder::<InterceptorEnum>::new()
            .build(source)
            .with_tracing()
            .collect()
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn token_text() {
        let source = "Simple text";
        let stream = token_stream(source);

        let tokens: Vec<_> = stream
            .iter()
            .filter_map(|(token, _)| token.as_text())
            .collect();

        assert_eq!(tokens, ["Simple text"]);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn token_code() {
        let source = "Use `println!` macro";
        let stream = token_stream(source);

        let tokens: Vec<_> = stream
            .iter()
            .filter_map(|(token, _)| token.as_code())
            .collect();

        assert_eq!(tokens, ["println!"]);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn token_soft_break() {
        let source = "Line 1\nLine 2";
        let stream = token_stream(source);

        let count = stream
            .iter()
            .filter(|(token, _)| token.is_soft_break())
            .count();

        assert_eq!(count, 1);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn token_hard_break() {
        let source = "Line 1  \nLine 2";
        let stream = token_stream(source);

        let count = stream
            .iter()
            .filter(|(token, _)| token.is_hard_break())
            .count();

        assert_eq!(count, 1);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn token_inline_math() {
        let source = "Formula $E=mc^2$ here";
        let stream = token_stream(source);

        let tokens: Vec<_> = stream
            .iter()
            .filter_map(|(token, _)| token.as_inline_math())
            .collect();

        assert_eq!(tokens, ["E=mc^2"]);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn token_display_math() {
        let source = "Formula $$E=mc^2$$ here";
        let stream = token_stream(source);

        let tokens: Vec<_> = stream
            .iter()
            .filter_map(|(token, _)| token.as_display_math())
            .collect();

        assert_eq!(tokens, ["E=mc^2"]);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn token_html() {
        let source = "<!-- Content -->";
        let stream = token_stream(source);

        let tokens: Vec<_> = stream
            .iter()
            .filter_map(|(token, _)| token.as_html())
            .collect();

        assert!(!tokens.is_empty());
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn token_inline_html() {
        let source = "Text <span>inline</span> text";
        let stream = token_stream(source);

        let tokens: Vec<_> = stream
            .iter()
            .filter_map(|(token, _)| token.as_inline_html())
            .collect();

        assert_eq!(tokens.len(), 2);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn token_rule() {
        let source = "Text\n\n---\n\nMore text";
        let stream = token_stream(source);

        let count = stream.iter().filter(|(token, _)| token.is_rule()).count();

        assert_eq!(count, 1);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn token_footnote_reference() {
        let source = "Text[^1] here";
        let stream = token_stream(source);

        let tokens: Vec<_> = stream
            .iter()
            .filter_map(|(token, _)| token.as_footnote_reference())
            .collect();

        assert_eq!(tokens, ["1"]);
    }
}
