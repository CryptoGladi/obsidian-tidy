mod footnote_index;

pub use footnote_index::{FootnoteIndex, FootnoteIndexExt};

use alloc::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FootnoteDefinition<'input> {
    pub(crate) label: Cow<'input, str>,
}

impl FootnoteDefinition<'_> {
    pub fn label(&self) -> &str {
        self.label.as_ref()
    }
}

crate::__private::impl_as_target_self!(FootnoteDefinition<'_>);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{
        InterceptorEnum, Tag, TagEnd, Token, TokenStreamBuilder, TracingTokenStreamExt,
    };
    use core::range::Range;
    use proptest::prelude::*;

    // ⚠️ CRITICAL: Footnotes require `old_footnotes(true)` to be parsed.
    // Without this option, `[^1]: text` is treated as plain paragraph text
    fn make_token_stream(source: &str) -> impl Iterator<Item = (Token<'_>, Range<usize>)> {
        TokenStreamBuilder::<InterceptorEnum>::new()
            .old_footnotes(true)
            .build(source)
            .with_tracing()
    }

    fn collect_tokens(source: &str) -> Vec<(Token<'_>, Range<usize>)> {
        make_token_stream(source).collect()
    }

    fn collect_footnote_definitions(source: &str) -> Vec<FootnoteDefinition<'_>> {
        let tokens = collect_tokens(source);

        tokens
            .into_iter()
            .filter_map(|(token, _)| token.into_start().and_then(Tag::into_footnote_definition))
            .collect()
    }

    fn count_start_footnote_definitions(source: &str) -> usize {
        collect_footnote_definitions(source).len()
    }

    fn count_end_footnote_definitions(source: &str) -> usize {
        let tokens = collect_tokens(source);
        tokens
            .into_iter()
            .filter(|(token, _)| token.as_end().is_some_and(TagEnd::is_footnote_definition))
            .count()
    }

    #[track_caller]
    fn assert_balanced(source: &str) {
        let starts = count_start_footnote_definitions(source);
        let ends = count_end_footnote_definitions(source);

        assert_eq!(
            starts, ends,
            "FootnoteDefinition tags must be balanced for source: `{source}`"
        );
    }

    macro_rules! assert_json_snapshot {
        ($tokens:ident) => {{
            let tokens: Vec<_> = $tokens
                .into_iter()
                .map(|(token, range)| (token, std::ops::Range::from(range)))
                .collect();
            insta::assert_json_snapshot!(tokens);
        }};
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn simple_footnote_definition() {
        let source = "[^1]: This is a footnote.";
        let definitions = collect_footnote_definitions(source);

        assert_eq!(definitions.len(), 1);
        assert_eq!(definitions[0].label(), "1");
        assert_balanced(source);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn footnote_definition_with_named_label() {
        let source = "[^note]: A named footnote reference.";
        let definitions = collect_footnote_definitions(source);

        assert_eq!(definitions.len(), 1);
        assert_eq!(definitions[0].label(), "note");
        assert_balanced(source);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn multiple_footnote_definitions() {
        let source = "[^1]: First footnote.
[^2]: Second footnote.
[^3]: Third footnote.";
        let definitions = collect_footnote_definitions(source);

        assert_eq!(definitions.len(), 3);
        assert_eq!(definitions[0].label(), "1");
        assert_eq!(definitions[1].label(), "2");
        assert_eq!(definitions[2].label(), "3");
        assert_balanced(source);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn footnote_definition_with_multiline_content() {
        let source = "[^long]: This is a long footnote
    that spans multiple lines
    with indented continuation.";
        let definitions = collect_footnote_definitions(source);

        assert_eq!(definitions.len(), 1);
        assert_eq!(definitions[0].label(), "long");
        assert_balanced(source);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn footnote_definition_with_formatting() {
        let source = "[^fmt]: A footnote with **bold** and *italic* text.";
        let definitions = collect_footnote_definitions(source);

        assert_eq!(definitions.len(), 1);
        assert_eq!(definitions[0].label(), "fmt");
        assert_balanced(source);

        // Verify that formatting tokens are present inside the footnote
        let tokens = collect_tokens(source);
        let has_strong = tokens
            .iter()
            .any(|(token, _)| matches!(token, Token::Start(Tag::Strong)));
        let has_emphasis = tokens
            .iter()
            .any(|(token, _)| matches!(token, Token::Start(Tag::Emphasis)));

        assert!(has_strong, "Should parse strong inside footnote definition");
        assert!(
            has_emphasis,
            "Should parse emphasis inside footnote definition"
        );
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn footnote_definition_with_code() {
        let source = "[^code]: Use `println!` for output.";
        let definitions = collect_footnote_definitions(source);

        // Verify that formatting tokens are present inside the footnote
        let tokens = collect_tokens(source);
        let has_code = tokens
            .iter()
            .any(|(token, _)| matches!(token, Token::Code(_)));

        assert!(has_code);
        assert_eq!(definitions.len(), 1);
        assert_eq!(definitions[0].label(), "code");
        assert_balanced(source);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn footnote_definition_with_link() {
        let source = "[^link]: See [Rust](https://rust-lang.org) for details.";
        let definitions = collect_footnote_definitions(source);

        assert_eq!(definitions.len(), 1);
        assert_eq!(definitions[0].label(), "link");
        assert_balanced(source);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn no_footnote_definitions_in_plain_text() {
        let source = "This is just regular text with [brackets] and (parens).";

        assert_eq!(count_start_footnote_definitions(source), 0);
        assert_eq!(count_end_footnote_definitions(source), 0);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn no_footnote_definitions_without_option() {
        // Without old_footnotes(true), pulldown-cmark treats [^1]: as plain text
        let source = "[^1]: This should NOT be a footnote.";
        let tokens = TokenStreamBuilder::<InterceptorEnum>::new()
            // ⚠️ old_footnotes NOT enabled
            .build(source)
            .collect::<Vec<_>>();

        let has_footnote = tokens
            .iter()
            .any(|(token, _)| token.as_start().is_some_and(Tag::is_footnote_definition));

        assert!(
            !has_footnote,
            "Footnote should NOT be parsed without old_footnotes option"
        );
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn empty_document() {
        let source = "";

        assert_eq!(count_start_footnote_definitions(source), 0);
        assert_eq!(count_end_footnote_definitions(source), 0);
    }

    // === Integration with References ===

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn footnote_reference_and_definition() {
        let source = r"Here is a footnote reference[^1].

[^1]: And here is the definition.";
        let definitions = collect_footnote_definitions(source);

        assert_eq!(definitions.len(), 1);
        assert_eq!(definitions[0].label(), "1");
        assert_balanced(source);

        // Verify that the reference token is also present
        let tokens = collect_tokens(source);
        let has_reference = tokens
            .iter()
            .any(|(token, _)| token.as_footnote_reference().is_some());

        assert!(has_reference, "Should have a FootnoteReference token");
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn multiple_references_single_definition() {
        let source = "First[^1] and second[^1] reference.

[^1]: Shared definition.";
        let definitions = collect_footnote_definitions(source);

        assert_eq!(definitions.len(), 1);
        assert_balanced(source);
    }

    // === Snapshot Tests ===

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn snapshot_simple() {
        let source = "Text[^1] here.

[^1]: Footnote content.";
        let tokens = collect_tokens(source);
        assert_json_snapshot!(tokens);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn snapshot_complex() {
        let source = "# Document

Paragraph with[^note] a reference.

[^note]: A **bold** footnote with `code` and [link](url).";
        let tokens = collect_tokens(source);
        assert_json_snapshot!(tokens);
    }

    // === Proptest ===

    fn footnote_content_strategy() -> impl Strategy<Value = String> {
        // The text must start with a visible character (not spaces/newlines)
        // to avoid being parsed as an indented code block.
        // Spec: https://spec.commonmark.org/0.31.2/#indented-code-blocks
        "[a-zA-Z0-9][a-zA-Z0-9 .,!?-]{0,20}"
    }

    proptest! {
        #[test]
        #[cfg_attr(miri, ignore)]
        fn prop_single_footnote_definition(
            label in "[a-zA-Z0-9]{1,10}", // TODO ADD UNICODE
            content in footnote_content_strategy()
        ) {
            let source = format!("[^{label}]: {content}");
            let definitions = collect_footnote_definitions(&source);

            prop_assert_eq!(
                definitions.len(), 1,
                "Should parse exactly one footnote definition: source = `{}`", source
            );
            prop_assert_eq!(
                definitions[0].label(), label.as_str(),
                "Label mismatch: source = `{}`", source
            );

            prop_assert_eq!(count_start_footnote_definitions(&source), 1);
            prop_assert_eq!(count_end_footnote_definitions(&source), 1);
        }

        #[test]
        #[cfg_attr(miri, ignore)]
        fn prop_balanced_tags(
            label in "[a-zA-Z0-9]{1,10}",
            content in footnote_content_strategy()
        ) {
            let source = format!("Text[^{label}].\n\n[^{label}]: {content}");
            let starts = count_start_footnote_definitions(&source);
            let ends = count_end_footnote_definitions(&source);

            prop_assert_eq!(
                starts, ends,
                "Tags must be balanced: source = `{}`", source
            );
        }

        #[test]
        #[cfg_attr(miri, ignore)]
        fn prop_multiple_definitions(
            label1 in "[a-z]{1,5}",
            label2 in "[a-z]{1,5}",
            content1 in footnote_content_strategy(),
            content2 in footnote_content_strategy()
        ) {
            // Ensure labels are different to avoid deduplication
            prop_assume!(label1 != label2);

            let source = format!(
                "[^{label1}]: {content1}\n\n[^{label2}]: {content2}"
            );
            let definitions = collect_footnote_definitions(&source);

            prop_assert_eq!(
                definitions.len(), 2,
                "Should parse two footnote definitions: source = `{}`", source
            );

            prop_assert_eq!(count_start_footnote_definitions(&source), 2);
            prop_assert_eq!(count_end_footnote_definitions(&source), 2);
        }
    }
}
