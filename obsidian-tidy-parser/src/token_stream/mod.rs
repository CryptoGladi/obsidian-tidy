mod interceptor;
mod lookahead;
mod markdown_lexer_adapter;
mod token;

pub use token::Token;

use crate::markdown_lexer::MarkdownLexer;
use interceptor::{Interceptor, InterceptorEnum, get_all_interceptors};
use lookahead::Lookahead;
use markdown_lexer_adapter::MarkdownLexerAdapter as LexerAdapter;
use std::range::Range;

pub struct TokenStream<'input> {
    lexer: Lookahead<LexerAdapter<'input>>,
    interceptors: Vec<InterceptorEnum>,
    source: &'input str,
}

impl<'input> TokenStream<'input> {
    pub fn new<I>(source: &'input str, lexer: MarkdownLexer<'input>, interceptors: I) -> Self
    where
        I: IntoIterator<Item = InterceptorEnum>,
    {
        let adapter = LexerAdapter::new(lexer);
        let lexer = Lookahead::new(adapter);

        Self {
            lexer,
            source,
            interceptors: interceptors.into_iter().collect(),
        }
    }

    pub fn new_with_all_interceptors(source: &'input str, lexer: MarkdownLexer<'input>) -> Self {
        let interceptors = get_all_interceptors();

        Self::new(source, lexer, interceptors)
    }
}

impl<'input> Iterator for TokenStream<'input> {
    type Item = (Token<'input>, Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.lexer.next()?;

        for interceptor in &mut self.interceptors {
            if let Some(d) = interceptor.try_intercept(self.source, &mut self.lexer, &current) {
                return Some(d);
            }
        }

        Some(current)
    }
}
