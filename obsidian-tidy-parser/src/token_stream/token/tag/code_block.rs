use alloc::borrow::Cow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CodeBlock<'input> {
    fenced: Option<Cow<'input, str>>,
}

impl<'input> From<pulldown_cmark::CodeBlockKind<'input>> for CodeBlock<'input> {
    fn from(kind: pulldown_cmark::CodeBlockKind<'input>) -> Self {
        let fenced = match kind {
            pulldown_cmark::CodeBlockKind::Fenced(fenced) => Some(fenced.into()),
            pulldown_cmark::CodeBlockKind::Indented => None,
        };

        CodeBlock { fenced }
    }
}
