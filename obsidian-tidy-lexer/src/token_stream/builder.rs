use super::TokenStream;
use crate::markdown_lexer::MarkdownLexerBuilder;
use crate::token_stream::InterceptorEnum;
use crate::token_stream::interceptor::get_all_interceptors;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenStreamBuilder<I = InterceptorEnum> {
    lexer: MarkdownLexerBuilder,
    interceptors: Vec<I>,
}

impl Default for TokenStreamBuilder<InterceptorEnum> {
    fn default() -> Self {
        Self {
            interceptors: get_all_interceptors(),
            lexer: MarkdownLexerBuilder::default(),
        }
    }
}

impl<I> TokenStreamBuilder<I> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            interceptors: Vec::new(),
            lexer: MarkdownLexerBuilder::default(),
        }
    }

    #[must_use]
    pub fn tables(mut self, enable: bool) -> Self {
        self.lexer = self.lexer.tables(enable);
        self
    }

    #[must_use]
    pub fn strikethrough(mut self, enable: bool) -> Self {
        self.lexer = self.lexer.strikethrough(enable);
        self
    }

    #[must_use]
    pub fn tasklists(mut self, enable: bool) -> Self {
        self.lexer = self.lexer.tasklists(enable);
        self
    }

    #[must_use]
    pub fn frontmatter(mut self, enable: bool) -> Self {
        self.lexer = self.lexer.frontmatter(enable);

        self
    }

    #[must_use]
    pub fn old_footnotes(mut self, enable: bool) -> Self {
        self.lexer = self.lexer.old_footnotes(enable);
        self
    }

    #[must_use]
    pub fn math(mut self, enable: bool) -> Self {
        self.lexer = self.lexer.math(enable);
        self
    }

    #[must_use]
    pub fn definition_list(mut self, enable: bool) -> Self {
        self.lexer = self.lexer.definition_list(enable);

        self
    }

    #[must_use]
    pub fn superscript(mut self, enable: bool) -> Self {
        self.lexer = self.lexer.superscript(enable);
        self
    }

    #[must_use]
    pub fn subscript(mut self, enable: bool) -> Self {
        self.lexer = self.lexer.subscript(enable);
        self
    }

    #[must_use]
    pub fn wikilinks(mut self, enable: bool) -> Self {
        self.lexer = self.lexer.wikilinks(enable);
        self
    }

    #[must_use]
    pub fn strict_markdown() -> Self {
        let lexer = MarkdownLexerBuilder::strict_markdown();

        Self {
            lexer,
            ..Self::new()
        }
    }

    #[must_use]
    pub fn add_interceptor(mut self, interceptor: I) -> Self {
        self.interceptors.push(interceptor);
        self
    }

    #[must_use]
    #[expect(
        clippy::elidable_lifetime_names,
        reason = "explicit 'input lifetime explicitly links TokenStream validity to the source buffer"
    )]
    pub fn build<'input>(self, source: &'input str) -> TokenStream<'input, I> {
        let lexer = self.lexer.build(source);

        TokenStream::new(source, lexer, self.interceptors)
    }
}
