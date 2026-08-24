use serde::{Deserialize, Serialize};

/// Represents the specific marker or delimiter used to denote a list item in Markdown source.
///
/// While Markdown renderers often treat `-`, `*`, and `+` as visually identical,
/// preserving the exact delimiter is crucial for linters and formatters to maintain
/// the original source code style and detect inconsistencies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ListDelimiter {
    /// Unordered list marker using a hyphen/dash (`-`).
    Dash,

    /// Unordered list marker using an asterisk (`*`).
    Asterisk,

    /// Unordered list marker using a plus sign (`+`).
    Plus,

    /// Ordered list marker using a period/dot after the number (e.g., `1.` or `42.`).
    Period,

    /// Ordered list marker using a closing parenthesis after the number (e.g., `1)` or `42)`).
    OneParen,
}

static_assertions::assert_impl_all!(ListDelimiter: Copy, Clone);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct List {
    /// The starting number for an ordered list (e.g., `1` in `1. Item`).
    /// Always `None` for unordered lists.
    pub(crate) start_number: Option<u64>,

    /// The specific marker or delimiter used in the source text
    /// (e.g., `-`, `*`, `+`, `.`, or `)`).
    ///
    /// ⚠️ **Important:** By default, this field is `None`, as the underlying lexer
    /// (`pulldown-cmark`) does not preserve this information for performance reasons.
    ///
    /// To ensure this field is correctly populated, you **must** use
    /// [`ListInterceptor`](crate::token_stream::interceptor::ListInterceptor)
    /// when constructing the [`TokenStream`](crate::token_stream::TokenStream).
    /// Without it, the value will always remain `None`.
    pub(crate) delimiter: Option<ListDelimiter>,
}

impl List {
    #[must_use]
    pub const fn new(start_number: Option<u64>) -> Self {
        Self {
            start_number,
            delimiter: None,
        }
    }

    #[must_use]
    pub const fn start_number(&self) -> Option<u64> {
        self.start_number
    }

    /// Returns the list delimiter.
    ///
    /// # Panics
    ///
    /// Panics if [`ListInterceptor`] has not been added to the [`TokenStreamBuilder`].
    /// If you are not sure whether the interceptor is attached, use [`Self::delimiter_opt`].
    ///
    /// [`ListInterceptor`]: crate::ListInterceptor
    /// [`TokenStreamBuilder`]: crate::TokenStreamBuilder
    #[must_use]
    #[track_caller]
    #[expect(clippy::expect_used)]
    pub const fn delimiter(&self) -> ListDelimiter {
        self.delimiter.expect(
            "List delimiter is missing. \
             Did you forget to add `ListInterceptor` to `TokenStreamBuilder`?",
        )
    }

    /// Safely returns the delimiter if it has been defined.
    #[must_use]
    pub const fn delimiter_opt(&self) -> Option<ListDelimiter> {
        self.delimiter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{InterceptorEnum, Token, TokenStreamBuilder, TracingTokenStreamExt};
    use core::range::Range;

    fn make_token_stream(source: &str) -> impl Iterator<Item = (Token<'_>, Range<usize>)> {
        TokenStreamBuilder::<InterceptorEnum>::new()
            .build(source)
            .with_tracing()
    }

    fn collect_tokens(source: &str) -> Vec<(Token<'_>, Range<usize>)> {
        let lexer = make_token_stream(source);
        lexer.into_iter().collect()
    }

    fn collect_start_lists(source: &str) -> Vec<List> {
        let tokens = collect_tokens(source);

        tokens
            .into_iter()
            .filter_map(|(token, _)| {
                let list = token.into_start().and_then(super::super::Tag::into_list)?;

                Some(list)
            })
            .collect()
    }

    fn count_start_lists(source: &str) -> usize {
        collect_start_lists(source).len()
    }

    fn count_end_lists(source: &str) -> usize {
        let tokens = collect_tokens(source);

        tokens
            .into_iter()
            .filter(|(token, _)| token.as_end().is_some_and(super::super::TagEnd::is_list))
            .count()
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn unordered_list_dash() {
        let source = "- Item 1\n- Item 2";
        let lists = collect_start_lists(source);

        assert_eq!(lists.len(), 1);
        assert_eq!(lists[0].start_number, None);
        assert_eq!(count_start_lists(source), count_end_lists(source));
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn unordered_list_asterisk_and_plus() {
        let source = "* Item 1\n+ Item 2";
        let lists = collect_start_lists(source);

        // These are different types of lists
        assert_eq!(lists.len(), 2);

        assert_eq!(lists[0].start_number, None);
        assert_eq!(lists[1].start_number, None);
        assert_eq!(count_start_lists(source), count_end_lists(source));
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn ordered_list_period() {
        let source = "1. First\n2. Second";
        let lists = collect_start_lists(source);

        assert_eq!(lists.len(), 1);

        assert_eq!(lists[0].start_number, Some(1));
        assert_eq!(count_start_lists(source), count_end_lists(source));
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn ordered_list_parenthesis() {
        let source = "1) First\n2) Second";
        let lists = collect_start_lists(source);

        // List one block!
        // TODO impl AST for List
        assert_eq!(lists.len(), 1);

        assert_eq!(lists[0].start_number, Some(1));
        assert_eq!(count_start_lists(source), count_end_lists(source));
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn ordered_list_large_number() {
        let source = "42. The answer to everything";
        let lists = collect_start_lists(source);

        assert_eq!(lists.len(), 1);
        assert_eq!(lists[0].start_number, Some(42));
        assert_eq!(count_start_lists(source), count_end_lists(source));
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn nested_lists_balanced_tags() {
        let source = "- Item 1\n  - Nested item 1.1\n  - Nested item 1.2\n- Item 2";

        assert_eq!(count_start_lists(source), 2);
        assert_eq!(count_end_lists(source), 2);

        let lists = collect_start_lists(source);
        assert!(lists.iter().all(|l| l.start_number.is_none()));
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn mixed_ordered_and_unordered() {
        let source = "1. Ordered\n- Unordered\n2. Ordered again";
        let lists = collect_start_lists(source);

        assert_eq!(lists.len(), 3);
        assert_eq!(lists[0].start_number, Some(1));
        assert_eq!(lists[1].start_number, None);
        assert_eq!(lists[2].start_number, Some(2));

        assert_eq!(count_start_lists(source), count_end_lists(source));
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn no_lists_in_plain_text() {
        let source = "Just some plain text.\nNo lists here.";

        assert_eq!(count_start_lists(source), 0);
        assert_eq!(count_end_lists(source), 0);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn empty_document() {
        let source = "";

        assert_eq!(count_start_lists(source), 0);
        assert_eq!(count_end_lists(source), 0);
    }
}
