use super::{InterceptResult, Interceptor, InterceptorEnum};
use crate::token_stream::lookahead::Lookahead;
use crate::token_stream::markdown_lexer_adapter::MarkdownLexerAdapter as LexerAdapter;
use crate::token_stream::token::{ListDelimiter, Tag, Token};
use core::range::Range;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ListInterceptor;

impl From<ListInterceptor> for InterceptorEnum {
    fn from(interceptor: ListInterceptor) -> Self {
        InterceptorEnum::ListInterceptor(interceptor)
    }
}

const fn detect_delimiter(source: &str, start: usize) -> Option<ListDelimiter> {
    let bytes = source.as_bytes();

    if start >= bytes.len() {
        return None;
    }

    match bytes[start] {
        b'-' => Some(ListDelimiter::Dash),
        b'*' => Some(ListDelimiter::Asterisk),
        b'+' => Some(ListDelimiter::Plus),
        b'0'..=b'9' => {
            let mut i = start;

            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }

            if i < bytes.len() {
                if bytes[i] == b'.' {
                    Some(ListDelimiter::Period)
                } else if bytes[i] == b')' {
                    Some(ListDelimiter::OneParen)
                } else {
                    None
                }
            } else {
                None
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
            tracing::warn!(
                "DELIMITER ALREADY EXISTS!\nCheck update pulldown-cmark and delete this interceptor"
            );
            return None;
        }

        let delimiter = detect_delimiter(source, range.start)?;
        list.delimiter = Some(delimiter);

        Some((Token::Start(Tag::List(list)), *range))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{TokenStreamBuilder, TracingTokenStreamExt};
    use crate::token_stream::token::List;

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
            .add_interceptor(InterceptorEnum::from(ListInterceptor))
            .build(source)
            .with_tracing();

        stream
            .filter_map(|(token, _)| token.into_start().and_then(|tag| tag.into_list()))
            .collect()
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn detects_unordered_dash() {
        let lists = collect_lists("- Item 1\n- Item 2");
        assert_eq!(lists.len(), 1);

        assert_eq!(lists[0].delimiter, Some(ListDelimiter::Dash));
        assert_eq!(lists[0].start_number, None);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn detects_unordered_asterisk() {
        let lists = collect_lists("* Item 1\n* Item 2");
        assert_eq!(lists.len(), 1);

        assert_eq!(lists[0].delimiter, Some(ListDelimiter::Asterisk));
        assert_eq!(lists[0].start_number, None);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn detects_unordered_plus() {
        let lists = collect_lists("+ Item 1\n+ Item 2");
        assert_eq!(lists.len(), 1);

        assert_eq!(lists[0].delimiter, Some(ListDelimiter::Plus));
        assert_eq!(lists[0].start_number, None);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn detects_ordered_paren() {
        let lists = collect_lists("1) First\n2) Second");
        assert_eq!(lists.len(), 1);

        assert_eq!(lists[0].delimiter, Some(ListDelimiter::OneParen));
        assert_eq!(lists[0].start_number, Some(1));
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn detects_large_numbers() {
        let lists = collect_lists("42. The answer");
        assert_eq!(lists.len(), 1);

        assert_eq!(lists[0].delimiter, Some(ListDelimiter::Period));
        assert_eq!(lists[0].start_number, Some(42));
    }
}
