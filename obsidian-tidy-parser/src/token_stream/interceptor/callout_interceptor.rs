use super::{InterceptResult, Interceptor};
use crate::token_stream::Token;
use crate::token_stream::lookahead::Lookahead;
use crate::token_stream::markdown_lexer_adapter::MarkdownLexerAdapter as LexerAdapter;
use crate::token_stream::token::{Callout, CalloutFoldable, Tag, TagEnd};
use std::borrow::Cow;
use std::range::Range;

#[derive(Debug, PartialEq, Eq)]
enum Call {
    BlockQuote,
    Callout,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct CalloutInterceptor {
    stack: Vec<Call>,
}

fn get_ascii_at_offset(s: &str, offset: usize) -> Option<char> {
    s.as_bytes()
        .get(offset)
        .copied()
        .filter(|b| b.is_ascii())
        .map(|b| b as char)
}

impl CalloutInterceptor {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    fn parse_foldable<'input>(
        bracket_close_range: Range<usize>,
        source: &'input str,
    ) -> Option<CalloutFoldable> {
        let offset_foldable = bracket_close_range.start + 1;
        if !source.is_char_boundary(offset_foldable) {
            return None;
        }

        let char = get_ascii_at_offset(source, offset_foldable)?;
        Some(CalloutFoldable::from(char))
    }

    fn check_bracket_open<'input>(bracket_open_token: &Token<'input>) -> Option<()> {
        let Token::Text(text) = bracket_open_token else {
            return None;
        };

        if text.as_ref() != "[" {
            return None;
        }

        Some(())
    }

    fn check_bracket_close(bracket_close_token: &Token<'_>) -> Option<()> {
        let Token::Text(text) = bracket_close_token else {
            return None;
        };

        if text.as_ref() != "]" {
            return None;
        }

        Some(())
    }

    fn parse_kind<'input>(
        kind_token: &Token<'input>,
        kind_range: Range<usize>,
        source: &'input str,
    ) -> Option<&'input str> {
        let Token::Text(text) = kind_token else {
            return None;
        };
        let kind_str = {
            text.as_ref().strip_prefix('!')?;

            let range = Range {
                start: kind_range.start + 1,
                end: kind_range.end,
            };

            // Fix lifetime
            // kind_str depent by 'guard
            // Now it is 'input!
            &source[range]
        };

        if kind_str.is_empty() {
            return None;
        }

        Some(kind_str)
    }

    fn parse_start_callout<'input>(
        &self,
        block_quote: Range<usize>,
        source: &'input str,
        lexer: &mut Lookahead<LexerAdapter<'input>>,
    ) -> InterceptResult<'input> {
        let guard = lexer.peek_many::<4>()?;

        let [
            (paragraph_token, paragraph_range),
            (bracket_open_token, bracket_open_range),
            (kind_token, kind_range),
            (bracket_close_token, bracket_close_range),
        ] = guard.data();

        // Check Start(Paragraph)
        let Token::Start(Tag::Paragraph) = paragraph_token else {
            return None;
        };

        // Check Text("[")
        Self::check_bracket_open(bracket_open_token)?;

        // Check Text("!kind")
        let kind_str = Self::parse_kind(kind_token, *kind_range, source)?;

        // Check Text("]")
        Self::check_bracket_close(bracket_close_token)?;

        // Check foldable
        let foldable = Self::parse_foldable(*bracket_close_range, source).unwrap_or_default();

        let start_callout = {
            let header_offset = Range {
                start: bracket_open_range.start,
                end: if foldable.is_none() {
                    bracket_close_range.end
                } else {
                    bracket_close_range.end + 1
                },
            };

            let callout = Callout {
                kind: Cow::Borrowed(kind_str),
                header_offset,
                foldable,
            };

            let token = Token::Start(Tag::Callout(callout));

            (token, block_quote)
        };

        let paragraph = (paragraph_token.clone(), *paragraph_range);
        let bracket_open = (bracket_open_token.clone(), *bracket_open_range);
        let kind = (kind_token.clone(), *kind_range);
        let bracket_close = (bracket_close_token.clone(), *bracket_close_range);

        guard.commit_returning_first([start_callout, paragraph, bracket_open, kind, bracket_close])
    }
}

impl<'input> Interceptor<'input> for CalloutInterceptor {
    fn try_intercept(
        &mut self,
        source: &'input str,
        lexer: &mut Lookahead<LexerAdapter<'input>>,
        current: &(Token<'input>, Range<usize>),
    ) -> InterceptResult<'input> {
        let current_token = &current.0;
        let current_range = current.1;

        if let Some(Tag::BlockQuote) = current_token.as_start() {
            if let Some(result) = self.parse_start_callout(current_range, source, lexer) {
                self.stack.push(Call::Callout);
                return Some(result);
            }

            self.stack.push(Call::BlockQuote);
        }

        if let Some(TagEnd::BlockQuote) = current_token.as_end() {
            // Теги обязаны быть сбалансированными!
            #[expect(clippy::expect_used)]
            let current = self.stack.pop().expect("unbalanced tags");

            let token = match current {
                Call::BlockQuote => Token::End(TagEnd::BlockQuote),
                Call::Callout => Token::End(TagEnd::Callout),
            };

            return Some((token, current_range));
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::markdown_lexer::MarkdownLexerBuilder;
    use crate::token_stream::TokenStream;
    use crate::vec_interceptor;
    use proptest::prelude::*;

    fn make_token_stream<'input>(source: &'input str) -> TokenStream<'input> {
        let lexer = MarkdownLexerBuilder::default().build(source);
        let interceptors = vec_interceptor![
            CalloutInterceptor => CalloutInterceptor::default()
        ];

        TokenStream::new(source, lexer, interceptors)
    }

    fn collect_tokens<'input>(source: &'input str) -> Vec<(Token<'input>, Range<usize>)> {
        let lexer = make_token_stream(source);
        lexer.into_iter().collect()
    }

    fn find_callout_starts<'a, 'input>(
        tokens: &'a [(Token<'input>, Range<usize>)],
    ) -> Vec<&'a Callout<'input>> {
        tokens
            .iter()
            .filter_map(|(token, _)| match token {
                Token::Start(Tag::Callout(callout)) => Some(callout),
                _ => None,
            })
            .collect()
    }

    fn count_end_callouts(tokens: &[(Token, Range<usize>)]) -> usize {
        tokens
            .iter()
            .filter(|(token, _)| match token {
                Token::End(TagEnd::Callout) => true,
                _ => false,
            })
            .count()
    }

    #[track_caller]
    fn count_callout_block<'a, 'input>(tokens: &'a [(Token<'input>, Range<usize>)]) -> usize {
        let starts = find_callout_starts(tokens).len();
        let ends = count_end_callouts(tokens);

        assert_eq!(starts, ends);

        starts
    }

    fn count_markdown_blockquote_start(tokens: &[(Token, Range<usize>)]) -> usize {
        tokens
            .iter()
            .filter(|(token, _)| matches!(token, Token::Start(Tag::BlockQuote)))
            .count()
    }

    fn count_markdown_blockquote_ends(tokens: &[(Token, Range<usize>)]) -> usize {
        tokens
            .iter()
            .filter(|(token, _)| matches!(token, Token::End(TagEnd::BlockQuote)))
            .count()
    }

    macro_rules! assert_json_snapshot {
        ($tokens:ident) => {{
            let tokens: Vec<_> = $tokens
                .into_iter()
                .map(|(token, range)| (token, std::ops::Range::from(range)))
                .collect();

            insta::assert_json_snapshot!(tokens);
        }};
    }

    #[track_caller]
    fn count_markdown_blockquote<'a, 'input>(tokens: &'a [(Token<'input>, Range<usize>)]) -> usize {
        let starts = count_markdown_blockquote_start(tokens);
        let ends = count_markdown_blockquote_ends(tokens);

        assert_eq!(starts, ends);

        starts
    }

    #[test]
    fn simple_callout_tip() {
        let source = "> [!tip]\n> Content";
        let tokens = collect_tokens(source);

        let callouts = find_callout_starts(&tokens);

        assert_eq!(callouts.len(), 1);
        assert_eq!(callouts[0].kind, "tip");
        assert_eq!(callouts[0].foldable, CalloutFoldable::None);

        assert_eq!(count_callout_block(&tokens), 1);
        assert_eq!(count_markdown_blockquote(&tokens), 0);

        assert_json_snapshot!(tokens);
    }

    #[test]
    fn callout_with_expanded_foldable() {
        let source = "> [!warning]+ Custom Title";
        let tokens = collect_tokens(source);

        let callouts = find_callout_starts(&tokens);

        assert_eq!(callouts.len(), 1);
        assert_eq!(callouts[0].kind, "warning");
        assert_eq!(callouts[0].foldable, CalloutFoldable::Expanded);

        assert_json_snapshot!(tokens);
    }

    #[test]
    fn callout_with_collapsed_foldable() {
        let source = "> [!note]- Hidden content";
        let tokens = collect_tokens(source);

        let callouts = find_callout_starts(&tokens);

        assert_eq!(callouts.len(), 1);
        assert_eq!(callouts[0].kind, "note");
        assert_eq!(callouts[0].foldable, CalloutFoldable::Collapsed);
    }

    #[test]
    fn callout_all_kinds() {
        // BUG Если я дам `--a`, то будет ошибка
        // Значит пока каждый kind должен является только английскими буквами
        // После исправления заставить proptest проверить наш синтаксис для любого
        // Или же это не ошибка? Можно ссылаться на правила приоритета Markdown
        let kinds = [
            "tip", "note", "warning", "danger", "info", "example", "quote", "bug", "abstract",
            "todo", "success", "question", "failure",
        ];

        for kind in kinds {
            let source = format!("> [!{}]", kind);
            let tokens = collect_tokens(&source);
            let callouts = find_callout_starts(&tokens);

            assert_eq!(callouts.len(), 1, "should parse callout for kind: {}", kind);
            assert_eq!(callouts[0].kind, kind, "kind mismatch for: {}", kind);
        }
    }

    proptest! {
        #[test]
        fn prop_callout_all_kinds(kind in "[A-Za-z ]{3,10}") {
            let kind = &kind;
            let source = format!("> [!{}]", kind);
            let tokens = collect_tokens(&source);
            let callouts = find_callout_starts(&tokens);

            prop_assert_eq!(callouts.len(), 1, "should parse callout for kind: {}", kind);
            prop_assert_eq!(callouts[0].kind.as_ref(), kind, "kind mismatch for: {}", kind);
        }
    }

    #[test]
    fn regular_blockquote_stays_blockquote() {
        let source = "> Just a regular quote";
        let tokens = collect_tokens(source);

        let callouts = find_callout_starts(&tokens);

        assert!(
            callouts.is_empty(),
            "regular blockquote should not become callout"
        );

        assert_eq!(count_markdown_blockquote(&tokens), 1);
        assert_eq!(count_callout_block(&tokens), 0);
    }

    #[test]
    fn blockquote_with_wrong_syntax() {
        let source = "> [tip]"; // not !
        let tokens = collect_tokens(source);

        let callouts = find_callout_starts(&tokens);

        assert!(callouts.is_empty());
        assert_eq!(count_markdown_blockquote(&tokens), 1);
        assert_eq!(count_callout_block(&tokens), 0);
    }

    #[test]
    fn blockquote_with_empty_kind() {
        let source = "> [!]";
        let tokens = collect_tokens(source);

        let callouts = find_callout_starts(&tokens);
        assert!(callouts.is_empty());

        assert_eq!(count_markdown_blockquote(&tokens), 1);
        assert_eq!(count_callout_block(&tokens), 0);
    }

    #[test]
    fn nested_blockquotes_counter_example() {
        let source = r#"> [!tip] Start!
> > > Super quote
> > End one quote
> But we still in Callout!"#;

        let tokens = collect_tokens(source);

        let callouts = find_callout_starts(&tokens);
        assert_eq!(callouts.len(), 1, "only outer blockquote is callout");
        assert_eq!(callouts[0].kind, "tip");

        assert_eq!(count_callout_block(&tokens), 1);
        assert_eq!(count_markdown_blockquote(&tokens), 2);

        assert_json_snapshot!(tokens);
    }

    #[test]
    fn tags_are_balanced() {
        let source = r#"> [!tip]
> Content 1
>
> > [!warning]
> > Content 2
>
> Content 3"#;

        let tokens = collect_tokens(source);

        let callout_starts = find_callout_starts(&tokens).len();
        let callout_ends = count_end_callouts(&tokens);

        assert_eq!(
            callout_starts, callout_ends,
            "callout starts and ends must be balanced"
        );
        assert_eq!(callout_starts, 2);

        assert_json_snapshot!(tokens);
    }

    proptest! {
        #[test]
        fn prop_tags_are_balanced(kind in "[A-Za-z ]{3,10}") {
            let source = format!("> [!{}]\n> Content\n", kind);
            let tokens = collect_tokens(&source);

            let starts = find_callout_starts(&tokens).len();
            let ends = count_end_callouts(&tokens);

            prop_assert_eq!(starts, ends);
        }
    }

    #[test]
    fn empty_document() {
        let source = "";
        let tokens = collect_tokens(source);

        assert_eq!(count_end_callouts(&tokens), 0);
        assert_eq!(count_markdown_blockquote(&tokens), 0);
    }

    #[test]
    fn multiple_sequential_callouts() {
        let source = r#"> [!tip] First

> [!warning] Second

> [!note] Third"#;

        let tokens = collect_tokens(source);
        let callouts = find_callout_starts(&tokens);

        assert_eq!(callouts.len(), 3);
        assert_eq!(callouts[0].kind, "tip");
        assert_eq!(callouts[1].kind, "warning");
        assert_eq!(callouts[2].kind, "note");

        assert_eq!(count_end_callouts(&tokens), 3);
    }

    #[test]
    fn parse_foldable_plus() {
        let source = "> [!tip]+";
        let tokens = collect_tokens(source);
        let callouts = find_callout_starts(&tokens);

        assert_eq!(callouts[0].foldable, CalloutFoldable::Expanded);
    }

    #[test]
    fn parse_foldable_minus() {
        let source = "> [!tip]-";
        let tokens = collect_tokens(source);
        let callouts = find_callout_starts(&tokens);

        assert_eq!(callouts[0].foldable, CalloutFoldable::Collapsed);
    }

    #[test]
    fn parse_foldable_none() {
        let source = "> [!tip]";
        let tokens = collect_tokens(source);
        let callouts = find_callout_starts(&tokens);

        assert_eq!(callouts[0].foldable, CalloutFoldable::None);
    }

    #[test]
    fn callout_with_markdown_inside() {
        let source = "> [!tip]\n> **Bold** and *italic*";
        let tokens = collect_tokens(source);

        let callouts = find_callout_starts(&tokens);
        assert_eq!(callouts.len(), 1);
        assert_eq!(callouts[0].kind, "tip");

        assert_json_snapshot!(tokens);
    }

    #[test]
    fn callout_with_code_inside() {
        let source = "> [!tip]\n> Use `println!` macro";
        let tokens = collect_tokens(source);

        let callouts = find_callout_starts(&tokens);
        assert_eq!(callouts.len(), 1);

        assert_json_snapshot!(tokens);
    }

    #[test]
    fn callout_offset_correctness() {
        let source = "> [!tip]+ Custom";
        let tokens = collect_tokens(source);

        let (_, range) = tokens
            .iter()
            .find(|(token, _)| matches!(token, Token::Start(Tag::Callout(_))))
            .expect("should find StartCallout");

        assert!(range.start < range.end, "range should be valid");

        let text = &source[range.clone()];
        assert!(
            text.contains("!tip"),
            "offset should cover callout content, got: {:?}",
            text
        );
    }
}
