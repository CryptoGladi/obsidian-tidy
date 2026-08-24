mod interceptor;

pub use interceptor::{TracingInterceptor, TracingInterceptorExt};

use super::TokenStream;
use crate::token_stream::interceptor::{Interceptor, InterceptorEnum};
use alloc::vec::Vec;
use tracing::{Span, trace_span};

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
    use crate::prelude::{InterceptResult, Interceptor, Token};
    use crate::token_stream::TokenStreamBuilder;
    use crate::token_stream::lookahead::Lookahead;
    use crate::token_stream::markdown_lexer_adapter::MarkdownLexerAdapter as LexerAdapter;
    use alloc::borrow::Cow;
    use alloc::string::{String, ToString};
    use core::range::Range;
    use proptest::prelude::*;

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
    #[cfg(not(miri))]
    #[tracing_test::traced_test]
    fn tracing_emits_logs() {
        let source = "# Hello World";
        let stream = TokenStreamBuilder::default().build(source);
        let mut traced_stream = stream.with_tracing();

        while traced_stream.next().is_some() {}

        assert!(logs_contain("Yielded token"));
        assert!(logs_contain("Hello") || logs_contain("World"));
    }

    #[test]
    #[cfg(not(miri))]
    #[tracing_test::traced_test]
    fn interceptor_logs_replacement() {
        let source = "some # hashtag";

        let interceptor = MockPredictableInterceptor::new(true);
        let stream = TokenStreamBuilder::new()
            .add_interceptor(interceptor)
            .build(source);

        let stream = stream.with_tracing();

        let tokens: Vec<_> = stream.collect();
        let has_replaced = tokens
            .iter()
            .any(|(t, _)| matches!(t, Token::Text(cow) if cow == "replaced_by_mock"));
        assert!(has_replaced, "Interceptor did not replace the token");

        assert!(logs_contain("Yielded token"));
        assert!(logs_contain("REPLACED") && logs_contain("replaced_by_mock"));
    }

    #[test]
    fn empty_stream_with_tracing() {
        let source = "";
        let stream = TokenStreamBuilder::default().build(source);
        let mut traced_stream = stream.with_tracing();

        assert!(traced_stream.next().is_none());
    }

    proptest! {
        #[test]
        #[cfg_attr(miri, ignore)]
        fn proptest_tracing_stream_identity(
            source in r"[a-zA-Z0-9 #>*-]{0,150}"
        ) {
            let original_stream = TokenStreamBuilder::default().build(&source);
            let original_tokens: Vec<_> = original_stream.collect();

            let traced_stream = TokenStreamBuilder::default().build(&source).with_tracing();
            let traced_tokens: Vec<_> = traced_stream.collect();

            prop_assert_eq!(original_tokens, traced_tokens);
        }
    }
}
