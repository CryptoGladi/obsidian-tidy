mod builder;
pub mod interceptor;
pub mod lookahead;
mod markdown_lexer_adapter;
pub mod token;

#[cfg(feature = "tracing")]
pub mod tracing;

pub use builder::TokenStreamBuilder;
pub use token::Token;

use crate::markdown_lexer::MarkdownLexer;
use alloc::vec::Vec;
use core::range::Range;
use interceptor::{Interceptor, InterceptorEnum, get_all_interceptors};
use lookahead::Lookahead;
use markdown_lexer_adapter::MarkdownLexerAdapter as LexerAdapter;

#[derive(Debug)]
pub struct TokenStream<'input, I = InterceptorEnum> {
    lexer: Lookahead<LexerAdapter<'input>>,
    interceptors: Vec<I>,
    source: &'input str,
}

impl<'input, I> TokenStream<'input, I> {
    pub fn new<Iter>(source: &'input str, lexer: MarkdownLexer<'input>, interceptors: Iter) -> Self
    where
        Iter: IntoIterator<Item = I>,
    {
        let adapter = LexerAdapter::new(lexer);
        let lexer = Lookahead::new(adapter);

        Self {
            lexer,
            source,
            interceptors: interceptors.into_iter().collect(),
        }
    }
}

impl<'input> TokenStream<'input, InterceptorEnum> {
    #[must_use]
    pub fn new_with_all_interceptors(source: &'input str, lexer: MarkdownLexer<'input>) -> Self {
        let interceptors = get_all_interceptors();

        Self::new(source, lexer, interceptors)
    }
}

impl<'input, I> Iterator for TokenStream<'input, I>
where
    I: Interceptor<'input>,
{
    type Item = (Token<'input>, Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.lexer.next()?;

        for interceptor in &mut self.interceptors {
            if let Some(replaced) =
                interceptor.try_intercept(self.source, &mut self.lexer, &current)
            {
                return Some(replaced);
            }
        }

        Some(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::InterceptResult;
    use std::sync::mpsc::{Receiver, Sender, channel};

    struct DynMockInterceptor {
        sender: Sender<i32>,
        count: i32,
    }

    impl DynMockInterceptor {
        fn new() -> (Self, Receiver<i32>) {
            let (sender, receiver) = channel();

            (Self { sender, count: 0 }, receiver)
        }
    }

    impl<'input> Interceptor<'input> for DynMockInterceptor {
        fn try_intercept(
            &mut self,
            _: &'input str,
            _: &mut Lookahead<LexerAdapter<'input>>,
            _: &(Token<'input>, Range<usize>),
        ) -> InterceptResult<'input> {
            self.count += 1;
            self.sender.send(self.count).unwrap();

            None
        }
    }

    #[test]
    fn dyn_check() {
        let (mut mock, receiver) = DynMockInterceptor::new();
        let dyn_mock_interceptor: &mut dyn Interceptor<'_> = &mut mock;

        let mut stream = TokenStreamBuilder::new()
            .add_interceptor(dyn_mock_interceptor)
            .build("empty");

        let _ = stream.next();
        assert_eq!(receiver.try_recv(), Ok(1));

        let _ = stream.next();
        assert_eq!(receiver.try_recv(), Ok(2));
    }

    #[test]
    fn boxed_check() {
        let (mock, receiver) = DynMockInterceptor::new();
        let boxed_mock_interceptor: Box<dyn Interceptor<'_>> = Box::new(mock);

        let mut stream = TokenStreamBuilder::new()
            .add_interceptor(boxed_mock_interceptor)
            .build("empty");

        let _ = stream.next();
        assert_eq!(receiver.try_recv(), Ok(1));

        let _ = stream.next();
        assert_eq!(receiver.try_recv(), Ok(2));
    }
}
