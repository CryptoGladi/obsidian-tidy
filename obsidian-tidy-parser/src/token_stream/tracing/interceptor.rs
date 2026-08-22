use crate::prelude::{InterceptResult, Interceptor, Token};
use crate::token_stream::lookahead::Lookahead;
use crate::token_stream::markdown_lexer_adapter::MarkdownLexerAdapter as LexerAdapter;
use alloc::string::{String, ToString};
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::markdown_lexer::MarkdownLexerBuilder;
    use crate::prelude::{InterceptResult, Interceptor, Token};
    use crate::token_stream::lookahead::Lookahead;
    use crate::token_stream::markdown_lexer_adapter::MarkdownLexerAdapter as LexerAdapter;
    use alloc::borrow::Cow;
    use alloc::string::String;
    use core::range::Range;

    #[derive(Debug)]
    struct MockInterceptor {
        should_replace: bool,
    }

    impl MockInterceptor {
        pub fn new(should_replace: bool) -> Self {
            Self { should_replace }
        }
    }

    impl core::fmt::Display for MockInterceptor {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "MockInterceptor")
        }
    }

    impl<'input> Interceptor<'input> for MockInterceptor {
        fn try_intercept(
            &mut self,
            _: &'input str,
            _: &mut Lookahead<LexerAdapter<'input>>,
            current: &(Token<'input>, Range<usize>),
        ) -> InterceptResult<'input> {
            let (_, range) = current;

            if self.should_replace {
                let replaced_token = Token::Text(Cow::Borrowed("REPLACED_TOKEN"));
                Some((replaced_token, *range))
            } else {
                None
            }
        }
    }

    fn create_adapter(source: &str) -> LexerAdapter<'_> {
        let lexer = MarkdownLexerBuilder::default().build(source);
        LexerAdapter::new(lexer)
    }

    fn try_intercept<'input>(
        interceptor: &mut dyn Interceptor<'input>,
        source: &'input str,
        current: (Token<'input>, Range<usize>),
    ) -> InterceptResult<'input> {
        let adapter = create_adapter(source);

        interceptor.try_intercept("dummy source", &mut Lookahead::new(adapter), &current)
    }

    #[test]
    #[cfg(not(miri))]
    #[tracing_test::traced_test]
    fn test_tracing_interceptor_logs_on_replacement() {
        let mock = MockInterceptor::new(true);
        let mut tracing_interceptor = TracingInterceptor::new(mock, "test_replacement");

        let text = "original";
        let dummy_token = Token::Text(Cow::Borrowed(text));
        let dummy_range = Range::from(0..text.len());
        let current = (dummy_token, dummy_range);

        let source = "dummy source";
        let result = try_intercept(&mut tracing_interceptor, source, current);

        assert!(result.is_some());
        let (replaced_token, replaced_range) = result.unwrap();

        assert!(matches!(replaced_token, Token::Text(cow) if cow == "REPLACED_TOKEN"));
        assert_eq!(replaced_range, dummy_range);

        assert!(
            logs_contain("REPLACED"),
            "Expected 'REPLACED' in logs, but it was missing"
        );

        assert!(
            logs_contain("test_replacement"),
            "Expected interceptor name 'test_replacement' in logs"
        );
    }

    #[test]
    #[cfg(not(miri))]
    #[tracing_test::traced_test]
    fn test_tracing_interceptor_silent_on_no_replacement() {
        let mock = MockInterceptor::new(false);
        let mut tracing_interceptor = TracingInterceptor::new(mock, "test_silent");

        let dummy_token = Token::Text(Cow::Borrowed("original"));
        let dummy_range = Range::from(0..8);
        let current = (dummy_token, dummy_range.clone());

        let result = try_intercept(&mut tracing_interceptor, "Super test", current);

        assert!(result.is_none());

        assert!(
            !logs_contain("REPLACED"),
            "Expected NO 'REPLACED' in logs, but it was found"
        );
    }

    #[test]
    #[cfg(not(miri))]
    #[tracing_test::traced_test]
    fn test_tracing_interceptor_ext_uses_display_name() {
        let mock = MockInterceptor::new(true);

        // Use extension trait
        let mut tracing_interceptor = mock.with_tracing();

        let dummy_token = Token::Text(Cow::Borrowed("original"));
        let dummy_range = Range::from(0..8);
        let current = (dummy_token, dummy_range.clone());

        let source = "dummy source";
        let result = try_intercept(&mut tracing_interceptor, source, current);

        assert!(result.is_some());

        assert!(
            logs_contain("MockInterceptor"),
            "Expected Display name 'MockInterceptor' in span logs"
        );

        assert!(logs_contain("REPLACED"));
    }
}
