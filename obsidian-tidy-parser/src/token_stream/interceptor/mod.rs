mod callout_interceptor;

pub use callout_interceptor::CalloutInterceptor;

use super::Token;
use crate::token_stream::Lookahead;
use crate::token_stream::markdown_lexer_adapter::MarkdownLexerAdapter as LexerAdapter;
use std::range::Range;

pub type InterceptResult<'input> = Option<(Token<'input>, Range<usize>)>;

pub trait Interceptor<'input> {
    fn try_intercept(
        &mut self,
        source: &'input str,
        lexer: &mut Lookahead<LexerAdapter<'input>>,
        current: &(Token<'input>, Range<usize>),
    ) -> InterceptResult<'input>;
}

static_assertions::assert_obj_safe!(Interceptor);

pub enum InterceptorEnum {
    CalloutInterceptor(callout_interceptor::CalloutInterceptor),
}

impl<'input> Interceptor<'input> for InterceptorEnum {
    fn try_intercept(
        &mut self,
        source: &'input str,
        lexer: &mut Lookahead<LexerAdapter<'input>>,
        current: &(Token<'input>, Range<usize>),
    ) -> InterceptResult<'input> {
        match self {
            InterceptorEnum::CalloutInterceptor(callout_interceptor) => {
                callout_interceptor.try_intercept(source, lexer, current)
            }
        }
    }
}

#[macro_export]
macro_rules! vec_interceptor {
    [] => {
        {
            Vec::<$crate::token_stream::interceptor::InterceptorEnum>::new()
        }
    };

    [ $( $variant:ident => $struct:expr ),* $(,)? ] => {
        {
            // Generate [ (), (), (), ... ].len()
            // It is zero const
            let capacity = [ $( vec_interceptor!(@replace $struct) ),* ].len();

            let mut v = Vec::with_capacity(capacity);

            $(v.push($crate::token_stream::interceptor::InterceptorEnum::$variant($struct));)*

            v
        }
    };

    (@replace $any:expr) => { () };
}

pub fn get_all_interceptors() -> Vec<InterceptorEnum> {
    vec_interceptor![
        CalloutInterceptor => CalloutInterceptor::default()
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use callout_interceptor::CalloutInterceptor;

    #[test]
    fn vec_interceptor_empty() {
        let interceptors = vec_interceptor![];
        assert!(interceptors.is_empty());
    }

    #[test]
    fn vec_interceptor_with_one_element() {
        let my_interceptor = CalloutInterceptor::new();
        let interceptors = vec_interceptor![CalloutInterceptor => my_interceptor];

        assert_eq!(interceptors.len(), 1);
    }

    #[test]
    fn vec_interceptor_with_many_elements() {
        let interceptor1 = CalloutInterceptor::new();
        let interceptor2 = CalloutInterceptor::new();
        let interceptor3 = CalloutInterceptor::new();

        let interceptors = vec_interceptor![
            CalloutInterceptor => interceptor1,
            CalloutInterceptor => interceptor2,
            CalloutInterceptor => interceptor3
        ];

        assert_eq!(interceptors.len(), 3);
    }

    #[test]
    fn vec_interceptor_trailing_comma() {
        let interceptor1 = CalloutInterceptor::new();
        let interceptor2 = CalloutInterceptor::new();
        let interceptor3 = CalloutInterceptor::new();

        let interceptors = vec_interceptor![
            CalloutInterceptor => interceptor1,
            CalloutInterceptor => interceptor2,
            CalloutInterceptor => interceptor3, // trailing comma
        ];

        assert_eq!(interceptors.len(), 3);
    }
}
