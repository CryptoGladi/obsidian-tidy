mod builder;

pub use builder::MarkdownLexerBuilder;

use pulldown_cmark::{Event as MarkEvent, Options as MarkOptions, Parser as MarkParser};
use std::range::Range;

#[derive(Debug)]
pub struct MarkdownLexer<'input> {
    inner: pulldown_cmark::OffsetIter<'input>,
}

impl<'input> MarkdownLexer<'input> {
    #[must_use]
    pub fn new(text: &'input str, options: MarkOptions) -> Self {
        let inner = MarkParser::new_ext(text, options);

        Self {
            inner: inner.into_offset_iter(),
        }
    }
}

impl<'input> Iterator for MarkdownLexer<'input> {
    type Item = (MarkEvent<'input>, Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|(event, range)| (event, range.into()))
    }
}
