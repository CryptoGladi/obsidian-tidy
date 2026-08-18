use crate::ast::ASTBuildExt;
use crate::ast::node::Node;
use crate::markdown_lexer::MarkdownLexerBuilder;
use crate::token_stream::TokenStream;
use ouroboros::self_referencing;
use std::sync::OnceLock;

#[self_referencing]
struct InnerDocument {
    source: String,

    #[borrows(source)]
    #[not_covariant]
    ast: OnceLock<Node<'this>>,
}

pub struct Document {
    inner: InnerDocument,
}

static_assertions::assert_impl_all!(Document: Send, Sync);

impl Document {
    pub fn new(source: impl Into<String>) -> Self {
        let inner = InnerDocumentBuilder {
            source: source.into(),
            ast_builder: |_| OnceLock::new(),
        }
        .build();

        Document { inner }
    }

    #[inline]
    pub fn source(&self) -> &str {
        self.inner.borrow_source()
    }

    pub fn ast<'ast>(&'ast self) -> &'ast Node<'ast> {
        self.inner.with(|fields| {
            fields.ast.get_or_init(|| {
                let lexer = MarkdownLexerBuilder::default().build(fields.source);
                let token_stream = TokenStream::new_with_all_interceptors(fields.source, lexer);

                token_stream.build_ast()
            })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        let text = "";
        let document = Document::new(text);
        let ast = document.ast();

        // Only root
        assert_eq!(ast.node_count(), 1);
    }
}
