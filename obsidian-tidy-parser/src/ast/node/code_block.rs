use alloc::borrow::Cow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeBlock<'ast> {
    fenced: Option<Cow<'ast, str>>,
}

impl<'input> From<crate::token_stream::token::CodeBlock<'input>> for CodeBlock<'input> {
    fn from(value: crate::token_stream::token::CodeBlock<'input>) -> Self {
        Self {
            fenced: value.fenced,
        }
    }
}

impl<'ast> CodeBlock<'ast> {
    #[must_use]
    pub const fn new(fenced: Option<Cow<'ast, str>>) -> Self {
        Self { fenced }
    }

    pub fn fenced(&'ast self) -> Option<&'ast str> {
        self.fenced.as_ref().map(Cow::as_ref)
    }
}

super::impl_node_as!(CodeBlock, CodeBlock<'_>);
