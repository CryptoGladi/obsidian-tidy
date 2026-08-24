use super::{InterceptResult, Interceptor, InterceptorEnum};
use crate::token_stream::lookahead::Lookahead;
use crate::token_stream::markdown_lexer_adapter::MarkdownLexerAdapter as LexerAdapter;
use crate::{ListDelimiter, Tag, Token};
use core::range::Range;

const MESSAGE_DELIMITER_ALREADY_EXISTS: &str =
    "DELIMITER ALREADY EXISTS! Check update pulldown-cmark and delete this interceptor";

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ListInterceptor;

impl From<ListInterceptor> for InterceptorEnum {
    fn from(interceptor: ListInterceptor) -> Self {
        InterceptorEnum::ListInterceptor(interceptor)
    }
}

/// For [`crate::TracingTokenStreamExt`] for tesing
#[cfg(feature = "tracing")]
#[cfg(test)]
impl core::fmt::Display for ListInterceptor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ListInterceptor")
    }
}

#[cold]
#[inline(never)]
fn warn_delimiter_already_exists() {
    #[cfg(feature = "tracing")]
    tracing::warn!("{MESSAGE_DELIMITER_ALREADY_EXISTS}");

    debug_assert!(false, "{MESSAGE_DELIMITER_ALREADY_EXISTS}");
}

fn detect_delimiter(source: &str, start: usize) -> Option<ListDelimiter> {
    let bytes = source.as_bytes();

    match bytes.get(start)? {
        b'-' => Some(ListDelimiter::Dash),
        b'*' => Some(ListDelimiter::Asterisk),
        b'+' => Some(ListDelimiter::Plus),

        // CommonMark spec allows leading zeros in ordered list markers.
        // See: https://spec.commonmark.org/0.31.2/#ordered-list-marker
        // "An ordered list marker is a sequence of 1–9 arabic digits (0-9)"
        // This means '0', '00', '05', '001' are all valid.
        //
        // Example: https://spec.commonmark.org/0.31.2/#example-268
        //
        // That's why detect_delimiter uses b'0'..=b'9' instead of b'1'..=b'9':
        // if we excluded '0', then "05. text" would not be recognized as a list.
        b'0'..=b'9' => {
            let mut i = start;

            while let Some(current) = bytes.get(i)
                && current.is_ascii_digit()
            {
                i += 1;
            }

            match bytes.get(i)? {
                b'.' => Some(ListDelimiter::Period),
                b')' => Some(ListDelimiter::OneParen),
                _ => None,
            }
        }
        _ => None,
    }
}

impl<'input> Interceptor<'input> for ListInterceptor {
    fn try_intercept(
        &mut self,
        source: &'input str,
        _: &mut Lookahead<LexerAdapter<'input>>,
        current: &(Token<'input>, Range<usize>),
    ) -> InterceptResult<'input> {
        let (token, range) = current;

        let &Token::Start(Tag::List(mut list)) = token else {
            return None;
        };

        if list.delimiter.is_some() {
            warn_delimiter_already_exists();
            return None;
        }

        let delimiter = detect_delimiter(source, range.start)?;
        list.delimiter = Some(delimiter);

        Some((Token::Start(Tag::List(list)), *range))
    }
}

static_assertions::assert_impl_all!(&mut ListInterceptor: Interceptor<'static>);
static_assertions::assert_impl_all!(alloc::boxed::Box<ListInterceptor>: Interceptor<'static>);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::List;
    use crate::markdown_lexer::MarkdownLexerBuilder;
    use crate::prelude::{TokenStreamBuilder, TracingTokenStreamExt};

    #[test]
    fn impl_display() {
        let interceptor = ListInterceptor;

        assert_eq!(interceptor.to_string(), "ListInterceptor");
    }

    #[test]
    fn impl_from_interceptor_enum() {
        let interceptor = ListInterceptor;

        assert!(matches!(
            InterceptorEnum::from(interceptor),
            InterceptorEnum::ListInterceptor(_)
        ));
    }

    fn collect_lists(source: &str) -> Vec<List> {
        let stream = TokenStreamBuilder::new()
            .add_interceptor(ListInterceptor)
            .build(source)
            .with_tracing();

        stream
            .filter_map(|(token, _)| token.into_start().and_then(crate::Tag::into_list))
            .collect()
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn delimiter_already_exists_graceful_degradation() {
        let mut interceptor = ListInterceptor;
        let source = "";
        let lexer = MarkdownLexerBuilder::default().build(source);
        let mut lookahead = Lookahead::new(LexerAdapter::new(lexer));

        // Creating already with delimiter
        let token = Token::Start(Tag::List(List {
            start_number: None,
            delimiter: Some(ListDelimiter::Dash),
        }));
        let range = Range::from(0..0);
        let current = (token, range);

        // In debug mode, debug_assert!(false, ...) MUST panic,
        // so that the developer immediately learns of the invariant violation.
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            interceptor.try_intercept(source, &mut lookahead, &current)
        }));

        #[cfg(debug_assertions)]
        {
            assert!(
                result.is_err(),
                "Expected panic from debug_assert in debug mode"
            );

            let panic_error = result.unwrap_err();

            let panic_message = panic_error
                .downcast_ref::<&str>()
                .copied()
                .or_else(|| panic_error.downcast_ref::<String>().map(String::as_str))
                .unwrap_or("Unknown panic message");

            #[cfg(not(miri))]
            assert!(logs_contain(MESSAGE_DELIMITER_ALREADY_EXISTS));

            assert_eq!(
                panic_message, MESSAGE_DELIMITER_ALREADY_EXISTS,
                "Unexpected panic message: {panic_message}",
            );
        }

        #[cfg(not(debug_assertions))]
        {
            assert!(result.is_ok(), "Should not panic in release mode");

            let intercepted = result.unwrap();

            assert_eq!(
                intercepted, None,
                "Should fallback to `None` end token gracefully"
            );
        }
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn test_snapshot() {
        let source = "- Item 1\n- Item 2";
        let stream = TokenStreamBuilder::new()
            .add_interceptor(ListInterceptor)
            .build(source)
            .with_tracing();

        let tokens: Vec<_> = stream
            .map(|(token, range)| (token, core::ops::Range::from(range)))
            .collect();

        #[cfg(not(miri))]
        insta::assert_json_snapshot!(tokens);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn detects_unordered_dash() {
        let lists = collect_lists("- Item 1\n- Item 2");
        assert_eq!(lists.len(), 1);

        assert_eq!(lists[0].delimiter_opt(), Some(ListDelimiter::Dash));
        assert_eq!(lists[0].start_number(), None);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn detects_unordered_asterisk() {
        let lists = collect_lists("* Item 1\n* Item 2");
        assert_eq!(lists.len(), 1);

        assert_eq!(lists[0].delimiter_opt(), Some(ListDelimiter::Asterisk));
        assert_eq!(lists[0].start_number(), None);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn detects_unordered_plus() {
        let lists = collect_lists("+ Item 1\n+ Item 2");
        assert_eq!(lists.len(), 1);

        assert_eq!(lists[0].delimiter_opt(), Some(ListDelimiter::Plus));
        assert_eq!(lists[0].start_number(), None);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn detects_ordered_paren() {
        let lists = collect_lists("1) First\n2) Second");
        assert_eq!(lists.len(), 1);

        assert_eq!(lists[0].delimiter_opt(), Some(ListDelimiter::OneParen));
        assert_eq!(lists[0].start_number(), Some(1));
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn detects_large_numbers() {
        let lists = collect_lists("42. The answer");
        assert_eq!(lists.len(), 1);

        assert_eq!(lists[0].delimiter_opt(), Some(ListDelimiter::Period));
        assert_eq!(lists[0].start_number(), Some(42));
    }

    /// `CommonMark` spec allows leading zeros in ordered list markers.
    /// See: <https://spec.commonmark.org/0.31.2/#ordered-list-marker>
    /// "An ordered list marker is a sequence of 1–9 arabic digits (0-9)"
    /// This means '0', '00', '05', '001' are all valid.
    ///
    /// Example: <https://spec.commonmark.org/0.31.2/#example-268>
    ///
    /// That's why `detect_delimiter` uses b'0'..=b'9' instead of b'1'..=b'9':
    /// if we excluded '0', then "05. text" would not be recognized as a list.
    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn ordered_list_with_leading_zeros() {
        let lists = collect_lists("005. The answer");
        assert_eq!(lists.len(), 1);

        assert_eq!(lists[0].delimiter, Some(ListDelimiter::Period));
        assert_eq!(lists[0].start_number, Some(5));
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn snapshot_ordered_list_with_leading_zeros() {
        let source = "005. The answer";
        let stream = TokenStreamBuilder::new()
            .add_interceptor(ListInterceptor)
            .build(source)
            .with_tracing();

        let tokens: Vec<_> = stream
            .map(|(token, range)| (token, core::ops::Range::from(range)))
            .collect();

        #[cfg(not(miri))]
        insta::assert_json_snapshot!(tokens);
    }
}
