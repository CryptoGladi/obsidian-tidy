mod callout_interceptor;
mod list_interceptor;

pub use callout_interceptor::CalloutInterceptor;
pub use list_interceptor::ListInterceptor;

use super::Token;
use crate::token_stream::Lookahead;
use crate::token_stream::markdown_lexer_adapter::MarkdownLexerAdapter as LexerAdapter;
use alloc::vec::Vec;
use core::range::Range;
use derive_more::IsVariant;

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

#[derive(Debug, Clone, PartialEq, Eq, IsVariant, strum::Display)]
pub enum InterceptorEnum {
    CalloutInterceptor(CalloutInterceptor),
    ListInterceptor(ListInterceptor),
}

impl<'input> Interceptor<'input> for InterceptorEnum {
    fn try_intercept(
        &mut self,
        source: &'input str,
        lexer: &mut Lookahead<LexerAdapter<'input>>,
        current: &(Token<'input>, Range<usize>),
    ) -> InterceptResult<'input> {
        match self {
            InterceptorEnum::CalloutInterceptor(interceptor) => {
                interceptor.try_intercept(source, lexer, current)
            }
            InterceptorEnum::ListInterceptor(interceptor) => {
                interceptor.try_intercept(source, lexer, current)
            }
        }
    }
}

#[macro_export]
macro_rules! vec_interceptor {
    [] => {
        {
            ::alloc::vec::Vec::<$crate::token_stream::interceptor::InterceptorEnum>::new()
        }
    };

    [ $( $struct:expr ),* $(,)? ] => {
        {
            // Generate [ (), (), (), ... ].len()
            // It is zero const
            let capacity = [ $( vec_interceptor!(@replace $struct) ),* ].len();

            let mut v = ::alloc::vec::Vec::with_capacity(capacity);

            $(v.push($crate::token_stream::interceptor::InterceptorEnum::from($struct));)*

            v
        }
    };

    (@replace $any:expr) => { () };
}

#[must_use]
#[cfg_attr(debug_assertions, track_caller)]
pub fn get_all_interceptors() -> Vec<InterceptorEnum> {
    let interceptors = vec_interceptor![CalloutInterceptor::default(), ListInterceptor];

    debug_assert!(
        {
            use alloc::string::ToString;
            let mut seen = alloc::collections::BTreeSet::new();

            interceptors.iter().all(|i| seen.insert(i.to_string()))
        },
        "Duplicated found in get_all_interceptors!"
    );

    interceptors
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
        let interceptors = vec_interceptor![my_interceptor];

        assert_eq!(interceptors.len(), 1);
    }

    #[test]
    fn vec_interceptor_with_many_elements() {
        let interceptor1 = CalloutInterceptor::new();
        let interceptor2 = CalloutInterceptor::new();
        let interceptor3 = CalloutInterceptor::new();

        let interceptors = vec_interceptor![interceptor1, interceptor2, interceptor3];

        assert_eq!(interceptors.len(), 3);
    }

    #[test]
    fn vec_interceptor_trailing_comma() {
        let interceptor1 = CalloutInterceptor::new();
        let interceptor2 = CalloutInterceptor::new();
        let interceptor3 = CalloutInterceptor::new();

        let interceptors = vec_interceptor![
            interceptor1,
            interceptor2,
            interceptor3, // trailing comma
        ];

        assert_eq!(interceptors.len(), 3);
    }
}
