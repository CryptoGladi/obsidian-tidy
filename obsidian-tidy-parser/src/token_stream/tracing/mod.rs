use super::TokenStream;
use crate::prelude::Token;
use crate::token_stream::interceptor::{InterceptResult, Interceptor, InterceptorEnum};
use crate::token_stream::lookahead::Lookahead;
use crate::token_stream::markdown_lexer_adapter::MarkdownLexerAdapter as LexerAdapter;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::range::Range;
use tracing::{Span, trace_span};

#[derive(Debug)]
pub struct TracingInterceptor<I> {
    inner: I,
    span: Span,
}

impl<I> TracingInterceptor<I> {
    pub fn new(inner: I, name: impl Into<String>) -> Self {
        let name = name.into();
        let span = trace_span!("interceptor", name = %name);

        Self { inner, span }
    }
}

impl<'input, I> Interceptor<'input> for TracingInterceptor<I>
where
    I: Interceptor<'input>,
{
    fn try_intercept(
        &mut self,
        source: &'input str,
        lexer: &mut Lookahead<LexerAdapter<'input>>,
        current: &(Token<'input>, Range<usize>),
    ) -> InterceptResult<'input> {
        let _guard = self.span.enter();

        if let Some(replaced) = self.inner.try_intercept(source, lexer, current) {
            tracing::debug!(?replaced, ?current, "REPLACED");

            return Some(replaced);
        }

        None
    }
}

pub trait TracingInterceptorExt<'input>: Interceptor<'input> + core::fmt::Display
where
    Self: Sized,
{
    fn with_tracing(self) -> TracingInterceptor<Self>;
}

impl<'input, I> TracingInterceptorExt<'input> for I
where
    I: Interceptor<'input> + core::fmt::Display,
{
    fn with_tracing(self) -> TracingInterceptor<Self> {
        let name = self.to_string();
        TracingInterceptor::new(self, name)
    }
}

pub struct TracingTokenStream<'input, I = InterceptorEnum> {
    inner: TokenStream<'input, TracingInterceptor<I>>,
    span: Span,
}

impl<'input, I> From<TokenStream<'input, I>> for TracingTokenStream<'input, I>
where
    I: Interceptor<'input> + core::fmt::Display,
{
    fn from(inner: TokenStream<'input, I>) -> Self {
        Self::new(inner)
    }
}

impl<'input, I> TracingTokenStream<'input, I>
where
    I: Interceptor<'input> + core::fmt::Display,
{
    pub fn new(inner: TokenStream<'input, I>) -> Self {
        let span = trace_span!("token_stream", source_len = inner.source.len());

        let tracing_interceptors: Vec<_> = inner
            .interceptors
            .into_iter()
            .map(TracingInterceptorExt::with_tracing)
            .collect();

        let inner = TokenStream {
            lexer: inner.lexer,
            source: inner.source,
            interceptors: tracing_interceptors,
        };

        Self { inner, span }
    }
}

impl<'input, I> Iterator for TracingTokenStream<'input, I>
where
    I: Interceptor<'input> + core::fmt::Display,
{
    type Item = <TokenStream<'input, I> as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let _guard = self.span.enter();

        // Simply call inner.next()!
        // The logs from TracingInterceptor will be processed automatically within it.
        let next = self.inner.next()?;

        let (token, range) = &next;
        let token_text = &self.inner.source[*range];

        tracing::trace!(%token_text, "Yielded token: {:?}", token);

        Some(next)
    }
}

pub trait TracingTokenStreamExt<'input, I> {
    fn with_tracing(self) -> TracingTokenStream<'input, I>;
}

impl<'input, I> TracingTokenStreamExt<'input, I> for TokenStream<'input, I>
where
    I: Interceptor<'input> + core::fmt::Display,
{
    fn with_tracing(self) -> TracingTokenStream<'input, I> {
        TracingTokenStream::from(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token_stream::TokenStreamBuilder;
    use alloc::borrow::Cow;

    #[derive(Debug)]
    struct MockPredictableInterceptor {
        should_replace: bool,
    }

    impl MockPredictableInterceptor {
        pub fn new(should_replace: bool) -> Self {
            Self { should_replace }
        }
    }

    impl core::fmt::Display for MockPredictableInterceptor {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "MockPredictableInterceptor")
        }
    }

    impl<'input> Interceptor<'input> for MockPredictableInterceptor {
        fn try_intercept(
            &mut self,
            _source: &'input str,
            _lexer: &mut Lookahead<LexerAdapter<'input>>,
            current: &(Token<'input>, Range<usize>),
        ) -> InterceptResult<'input> {
            let (_, range) = current;

            if self.should_replace {
                self.should_replace = false;

                let mock_token = Token::Text(Cow::Borrowed("replaced_by_mock"));
                let mock_range = *range;

                return Some((mock_token, mock_range));
            }

            None
        }
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn test_interceptor_logs_replacement() {
        let source = "some # hashtag";

        let interceptor = MockPredictableInterceptor::new(true);
        let stream = TokenStreamBuilder::new()
            .add_interceptor(interceptor)
            .build(source);

        let mut stream = stream.with_tracing();

        while stream.next().is_some() {}

        assert!(logs_contain("Yielded token"));
        assert!(logs_contain("REPLACED"));
    }
}
