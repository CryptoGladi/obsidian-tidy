use super::Token;
use crate::markdown_lexer::MarkdownLexer;
use core::range::Range;

#[derive(Debug)]
pub struct MarkdownLexerAdapter<'input> {
    inner: MarkdownLexer<'input>,
}

impl<'input> MarkdownLexerAdapter<'input> {
    pub const fn new(inner: MarkdownLexer<'input>) -> Self {
        Self { inner }
    }
}

impl<'input> Iterator for MarkdownLexerAdapter<'input> {
    type Item = (Token<'input>, Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|(event, range)| (Token::from(event), range))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::markdown_lexer::MarkdownLexerBuilder;
    use alloc::string::String;
    use alloc::vec::Vec;
    use proptest::prelude::*;
    use pulldown_cmark::Event as MarkEvent;

    #[test]
    fn empty_text() {
        let lexer = MarkdownLexerBuilder::default().build("");
        let mut adapter = MarkdownLexerAdapter::new(lexer);

        assert!(adapter.next().is_none())
    }

    type AdapterWithRange<'input> = (Token<'input>, Range<usize>);
    type LexerWithRange<'input> = (MarkEvent<'input>, Range<usize>);

    #[track_caller]
    fn impl_eq_adapter_and_original<'input>(
        markdown: &'input str,
    ) -> Option<(AdapterWithRange<'input>, LexerWithRange<'input>)> {
        let adapter = {
            let lexer = MarkdownLexerBuilder::default().build(markdown);
            MarkdownLexerAdapter::new(lexer)
        };

        let lexer = MarkdownLexerBuilder::default().build(markdown);

        let adapter_collect: Vec<_> = adapter.into_iter().collect();
        let lexer_collect: Vec<_> = lexer.collect();

        for (adapter, lexer) in adapter_collect.into_iter().zip(lexer_collect) {
            let (token, range_token) = adapter.clone();
            let (mark_event, range_mark_event) = lexer.clone();

            let any = token == mark_event.into()
                && range_token.start == range_mark_event.start
                && range_token.end == range_mark_event.end;

            if !any {
                return Some((adapter, lexer));
            }
        }

        None
    }

    #[test]
    fn eq_adapter_and_original() {
        let markdowns = ["123", "**bold** text", "# Header\nText"];

        for markdown in markdowns {
            if let Some((token, adapter)) = impl_eq_adapter_and_original(markdown) {
                panic!(
                    "Error markdown: `{}`: token`{:?}`, adapter: `{:?}`",
                    markdown, token, adapter
                );
            }
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
        #[cfg_attr(miri, ignore)]
        fn prop_eq_adapter_and_original(markdown in markdown()) {
            if let Some((token, adapter)) = impl_eq_adapter_and_original(&markdown) {
                prop_assert!(false,
                    "Error markdown: `{}`: token`{:?}`, adapter: `{:?}`",
                    markdown, token, adapter
                );
            }
        }
    }
}
