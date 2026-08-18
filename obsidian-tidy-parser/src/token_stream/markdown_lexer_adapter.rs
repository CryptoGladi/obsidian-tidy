use super::Token;
use crate::markdown_lexer::MarkdownLexer;
use std::range::Range;

pub struct MarkdownLexerAdapter<'input> {
    inner: MarkdownLexer<'input>,
}

impl<'input> MarkdownLexerAdapter<'input> {
    pub fn new(inner: MarkdownLexer<'input>) -> Self {
        Self { inner }
    }
}

impl<'input> Iterator for MarkdownLexerAdapter<'input> {
    type Item = (Token<'input>, Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|(event, range)| (Token::Markdown(event), range))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::markdown_lexer::MarkdownLexerBuilder;
    use proptest::prelude::*;

    #[test]
    fn always_token_markdown() {
        let markdowns = ["123", "**bold** text", "# Header\nText"];

        for markdown in markdowns {
            let lexer = MarkdownLexerBuilder::default().build(markdown);
            let adapter = MarkdownLexerAdapter::new(lexer);

            assert!(
                adapter.into_iter().any(|(token, _)| token.is_markdown()),
                "Error parse markdown: `{}`",
                markdown
            );
        }
    }

    #[test]
    fn empty_text() {
        let lexer = MarkdownLexerBuilder::default().build("");
        let mut adapter = MarkdownLexerAdapter::new(lexer);

        assert!(adapter.next().is_none())
    }

    #[test]
    fn eq_adapter_and_original() {
        let markdowns = ["123", "**bold** text", "# Header\nText"];

        for markdown in markdowns {
            let adapter = {
                let lexer = MarkdownLexerBuilder::default().build(markdown);
                MarkdownLexerAdapter::new(lexer)
            };

            let lexer = MarkdownLexerBuilder::default().build(markdown);

            let adapter_collect: Vec<_> = adapter
                .into_iter()
                .map(|(token, event)| (token.as_markdown().unwrap().clone(), event))
                .collect();

            let lexer_collect: Vec<_> = lexer.collect();

            assert_eq!(
                adapter_collect, lexer_collect,
                "Error markdown: `{}`",
                markdown
            );
        }
    }

    fn markdown_block() -> impl Strategy<Value = String> {
        prop_oneof![
            r"[#]{1,6} [a-zA-Z0-9 ]{5,15}\n",
            r"\*\*[a-zA-Z0-9 ]{5,15}\*\*\n",
            r"\*[a-zA-Z0-9 ]{5,15}\*\n",
            r"[a-zA-Z0-9 ]{5,20}\n",
        ]
    }

    fn markdown() -> impl Strategy<Value = String> {
        let blocks = markdown_block();
        let full = prop::collection::vec(blocks, 3..7);

        full.prop_map(|vec_of_strings| vec_of_strings.join("\n"))
    }

    proptest! {
        #[test]
        fn prop_always_token_markdown(markdown in markdown()) {
            let lexer = MarkdownLexerBuilder::default().build(&markdown);
            let adapter = MarkdownLexerAdapter::new(lexer);

            prop_assert!(adapter.into_iter().any(|(token, _)| token.is_markdown()));
        }

        #[test]
        fn prop_eq_adapter_and_original(markdown in markdown()) {
            let adapter = {
                let lexer = MarkdownLexerBuilder::default().build(&markdown);
                MarkdownLexerAdapter::new(lexer)
            };

            let lexer = MarkdownLexerBuilder::default().build(&markdown);

            let adapter_collect: Vec<_> = adapter
                .into_iter()
                .map(|(token, event)| (token.as_markdown().unwrap().clone(), event))
                .collect();

            let lexer_collect: Vec<_> = lexer.collect();

            prop_assert_eq!(adapter_collect, lexer_collect);
        }
    }
}
