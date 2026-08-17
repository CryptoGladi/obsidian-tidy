mod callout;
mod lookhead;
mod token;

pub use token::Token;

use crate::markdown_lexer::{MarkdownLexer, MarkdownLexerBuilder};
use lookhead::Lookahead;
use pulldown_cmark::{Event as MarkEvent, Tag as MarkTag};
use std::range::Range;

pub struct TokenStream<'input> {
    lexer: Lookahead<MarkdownLexer<'input>>,
    source: &'input str,
}

impl<'input> TokenStream<'input> {
    pub fn new(source: &'input str, builder: MarkdownLexerBuilder) -> Self {
        let lexer = builder.build(source);
        let lexer = Lookahead::new(lexer);

        Self { lexer, source }
    }
}

impl<'input> Iterator for TokenStream<'input> {
    type Item = (Token<'input>, Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        let (event, offset) = self.lexer.next()?;

        // Проверка и возраст Callout
        // С помощью commit_with
        // Ведь Start(Paragraph) мы также обязаны сделать!

        Some((Token::Markdown(event), offset))
    }
}
