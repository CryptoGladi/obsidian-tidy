use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ListDelimiter {
    /// Маркер тире `-`
    Dash,

    /// Маркер звёздочка `*`
    Asterisk,

    /// Маркер плюс `+`
    Plus,

    /// Точка после цифры `1.`
    Period,

    /// Скобка после цифры `1)`
    OneParen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct List {
    pub start_number: Option<u64>,

    // Use interceptor!
    pub delimiter: Option<ListDelimiter>,
}

impl List {
    #[must_use]
    pub const fn new(start_number: Option<u64>) -> Self {
        Self {
            start_number,
            delimiter: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{InterceptorEnum, Token, TokenStreamBuilder, TracingTokenStreamExt};
    use core::range::Range;

    fn make_token_stream<'input>(
        source: &'input str,
    ) -> impl Iterator<Item = (Token<'input>, Range<usize>)> {
        TokenStreamBuilder::<InterceptorEnum>::new()
            .build(source)
            .with_tracing()
    }

    fn collect_tokens<'input>(source: &'input str) -> Vec<(Token<'input>, Range<usize>)> {
        let lexer = make_token_stream(source);
        lexer.into_iter().collect()
    }

    fn collect_start_lists(source: &str) -> Vec<List> {
        let tokens = collect_tokens(source);

        tokens
            .into_iter()
            .filter_map(|(token, _)| {
                let list = token.into_start().and_then(|tag| tag.into_list())?;

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
            .filter(|(token, _)| token.as_end().is_some_and(|tag| tag.is_list()))
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
