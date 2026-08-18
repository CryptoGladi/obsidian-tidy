use crate::prelude::{ASTBuildExt, Node, TokenStream, TokenStreamBuilder};
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

    pub fn token_stream(&self) -> TokenStream<'_> {
        self.inner
            .with_source(|source| TokenStreamBuilder::default().build(source))
    }

    pub fn ast(&self) -> &Node<'_> {
        self.inner.with(|fields| {
            fields.ast.get_or_init(|| {
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
