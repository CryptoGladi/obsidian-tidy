use alloc::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CodeBlock<'input> {
    fenced: Option<Cow<'input, str>>,
}

crate::__private::impl_as_target_self!(CodeBlock<'_>);

impl<'input> From<pulldown_cmark::CodeBlockKind<'input>> for CodeBlock<'input> {
    fn from(kind: pulldown_cmark::CodeBlockKind<'input>) -> Self {
        let fenced = match kind {
            pulldown_cmark::CodeBlockKind::Fenced(fenced) => Some(fenced.into()),
            pulldown_cmark::CodeBlockKind::Indented => None,
        };

        CodeBlock { fenced }
    }
}

impl CodeBlock<'_> {
    pub fn fenced(&self) -> Option<&str> {
        self.fenced.as_ref().map(Cow::as_ref)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{
        InterceptorEnum, Tag, TagEnd, Token, TokenStreamBuilder, TracingTokenStreamExt,
    };
    use alloc::borrow::Cow;
    use core::range::Range;

    // --- Helpers ---

    fn make_token_stream(source: &str) -> impl Iterator<Item = (Token<'_>, Range<usize>)> {
        TokenStreamBuilder::<InterceptorEnum>::new()
            .build(source)
            .with_tracing()
    }

    fn collect_tokens(source: &str) -> Vec<(Token<'_>, Range<usize>)> {
        make_token_stream(source).collect()
    }

    fn collect_code_blocks(source: &str) -> Vec<CodeBlock<'_>> {
        let tokens = collect_tokens(source);

        tokens
            .into_iter()
            .filter_map(|(token, _)| token.into_start().and_then(Tag::into_code_block))
            .collect()
    }

    fn count_start_code_blocks(source: &str) -> usize {
        collect_code_blocks(source).len()
    }

    fn count_end_code_blocks(source: &str) -> usize {
        let tokens = collect_tokens(source);
        tokens
            .into_iter()
            .filter(|(token, _)| token.as_end().is_some_and(TagEnd::is_code_block))
            .count()
    }

    // --- Unit Tests for `From` trait ---

    #[test]
    fn from_pulldown_cmark_fenced() {
        let kind = pulldown_cmark::CodeBlockKind::Fenced("rust".into());
        let code_block = CodeBlock::from(kind);
        assert_eq!(code_block.fenced, Some(Cow::Borrowed("rust")));
    }

    #[test]
    fn from_pulldown_cmark_fenced_empty() {
        let kind = pulldown_cmark::CodeBlockKind::Fenced("".into());
        let code_block = CodeBlock::from(kind);
        assert_eq!(code_block.fenced, Some(Cow::Borrowed("")));
    }

    #[test]
    fn from_pulldown_cmark_indented() {
        let kind = pulldown_cmark::CodeBlockKind::Indented;
        let code_block = CodeBlock::from(kind);
        assert_eq!(code_block.fenced, None);
    }

    // --- Integration Tests via TokenStream ---

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn fenced_code_block_with_language() {
        let source = "```rust\nprintln!(\"hello\");\n```";
        let blocks = collect_code_blocks(source);

        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].fenced, Some(Cow::Borrowed("rust")));
        assert_eq!(
            count_start_code_blocks(source),
            count_end_code_blocks(source)
        );
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn fenced_code_block_without_language() {
        let source = "```\nsome plain code\n```";
        let blocks = collect_code_blocks(source);

        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].fenced, Some(Cow::Borrowed("")));
        assert_eq!(
            count_start_code_blocks(source),
            count_end_code_blocks(source)
        );
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn indented_code_block() {
        let source = "    let x = 1;\n    let y = 2;";
        let blocks = collect_code_blocks(source);

        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].fenced, None);
        assert_eq!(
            count_start_code_blocks(source),
            count_end_code_blocks(source)
        );
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn fenced_code_block_with_complex_info_string() {
        let source = "```rust,ignore,should_panic\nfn main() {}\n```";
        let blocks = collect_code_blocks(source);

        assert_eq!(blocks.len(), 1);
        assert_eq!(
            blocks[0].fenced,
            Some(Cow::Borrowed("rust,ignore,should_panic"))
        );
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn multiple_code_blocks_mixed() {
        let source = "```rust\nfn a() {}\n```\n\nSome text\n\n    let b = 2;";

        assert_eq!(count_start_code_blocks(source), 2);
        assert_eq!(count_end_code_blocks(source), 2);

        let blocks = collect_code_blocks(source);
        assert_eq!(blocks[0].fenced, Some(Cow::Borrowed("rust")));
        assert_eq!(blocks[1].fenced, None);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn empty_document_has_no_code_blocks() {
        let source = "";

        assert_eq!(count_start_code_blocks(source), 0);
        assert_eq!(count_end_code_blocks(source), 0);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn plain_text_has_no_code_blocks() {
        let source = "This is just a regular text with `inline code`, not a block.";

        assert_eq!(count_start_code_blocks(source), 0);
        assert_eq!(count_end_code_blocks(source), 0);
    }
}
