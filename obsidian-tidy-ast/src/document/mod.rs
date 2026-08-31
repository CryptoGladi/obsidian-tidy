use crate::prelude::{ASTBuildExt, Node};
use alloc::string::String;
use obsidian_tidy_lexer::{TokenStream, TokenStreamBuilder};
use ouroboros::self_referencing;

#[cfg(not(feature = "no_std"))]
pub use std::sync::OnceLock;

#[cfg(feature = "no_std")]
pub use spin::Once as OnceLock;

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

    pub fn token_stream(&self) -> TokenStream<'_> {
        self.inner
            .with_source(|source| TokenStreamBuilder::default().build(source))
    }

    #[cfg(not(feature = "no_std"))]
    pub fn ast(&self) -> &Node<'_> {
        self.inner.with(|fields| {
            fields.ast.get_or_init(|| {
                let token_stream = TokenStreamBuilder::default().build(fields.source);

                token_stream.build_ast()
            })
        })
    }

    #[cfg(feature = "no_std")]
    pub fn ast(&self) -> &Node<'_> {
        self.inner.with(|fields| {
            fields.ast.call_once(|| {
                let token_stream = TokenStreamBuilder::default().build(fields.source);

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
