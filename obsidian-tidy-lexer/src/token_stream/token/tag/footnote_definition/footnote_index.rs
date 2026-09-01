use crate::prelude::{FootnoteDefinition, Tag, TagEnd, Token};
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::range::Range;

#[cold]
#[inline(never)]
fn warn_malformed_slice_or_missing_end() {
    const MSG: &str = "Footnote definition has no matching End tag in the provided slice!";

    #[cfg(feature = "tracing")]
    tracing::error!("{MSG}");

    debug_assert!(false, "{MSG}");
}

#[cold]
#[inline(never)]
fn warn_footnote_already_exists(label: &str) {
    #[cfg(feature = "tracing")]
    tracing::error!("Footnote definition with label '{label}' already exists!");

    debug_assert!(
        false,
        "Footnote definition with label '{label}' already exists!"
    );
}

/// An index for fast bidirectional lookup between footnote references and definitions.
///
/// Built once after collecting tokens, then queried many times.
/// Uses `BTreeMap` for `no_std` compatibility.
///
/// # Complexity
/// - Build: $O(n \log n)$ where $n$ is the number of tokens
/// - Lookup: $O(\log k)$ where $k$ is the number of unique footnote labels
pub struct FootnoteIndex<'tokens, 'input> {
    /// label → definition token (one per label)
    definitions: BTreeMap<&'tokens str, &'tokens [(Token<'input>, Range<usize>)]>,

    /// label → reference tokens (many per label)
    references: BTreeMap<&'tokens str, Vec<&'tokens (Token<'input>, Range<usize>)>>,
}

impl<'tokens, 'input> FootnoteIndex<'tokens, 'input> {
    pub fn build(tokens: &'tokens [(Token<'input>, Range<usize>)]) -> Self {
        let mut definitions = BTreeMap::new();
        let mut references: BTreeMap<&str, Vec<_>> = BTreeMap::new();

        let mut iter = tokens.iter().enumerate();
        while let Some((i, token_pair)) = iter.next() {
            match &token_pair.0 {
                Token::Start(Tag::FootnoteDefinition(def)) => {
                    definitions
                        .entry(def.label())
                        .and_modify(|_| warn_footnote_already_exists(def.label()))
                        .or_insert_with(|| {
                            let tail = &tokens[i..];

                            let body_len = tail
                                .iter()
                                .position(|(t, _)| {
                                    matches!(t, Token::End(TagEnd::FootnoteDefinition))
                                })
                                .map(|pos| pos + 1)
                                .unwrap_or_else(|| {
                                    warn_malformed_slice_or_missing_end();

                                    tail.len()
                                });

                            iter.nth(body_len.saturating_mul(2)); // Полезная оптимизация

                            &tail[..body_len]
                        });
                }
                Token::FootnoteReference(label) => {
                    references
                        .entry(label.as_ref())
                        .or_default()
                        .push(token_pair);
                }
                _ => {}
            }
        }

        Self {
            definitions,
            references,
        }
    }

    /// Returns the **full slice** of tokens for a given footnote,
    /// including the opening `Start` and closing `End` tags.
    #[inline]
    pub fn full_slice_by_label(
        &self,
        label: &str,
    ) -> Option<&'tokens [(Token<'input>, Range<usize>)]> {
        self.definitions.get(label).copied()
    }

    #[inline(always)]
    fn strip_definition_tags(
        slice: &'tokens [(Token<'input>, Range<usize>)],
    ) -> &'tokens [(Token<'input>, Range<usize>)] {
        slice.get(1..slice.len().saturating_sub(1)).unwrap_or(&[])
    }

    /// Returns **only the inner content** of the footnote definition.
    ///
    /// Strips the surrounding `Start` and `End` tags. This is the **primary method**
    /// for rendering the footnote's body inside your Markdown HTML generator.
    #[inline]
    pub fn content_by_label(
        &self,
        label: &str,
    ) -> Option<&'tokens [(Token<'input>, Range<usize>)]> {
        self.definitions
            .get(label)
            .map(|&slice| Self::strip_definition_tags(slice))
    }

    /// Returns the raw `FootnoteDefinition` metadata (e.g., to read the original label).
    pub fn definition_meta(&self, label: &str) -> Option<&'tokens FootnoteDefinition<'input>> {
        self.definitions.get(label).and_then(|slice| match slice {
            [(Token::Start(Tag::FootnoteDefinition(def)), _), ..] => Some(def),
            _ => None,
        })
    }

    /// Returns an iterator over all registered footnote definitions.
    ///
    /// Yields pairs of `(label, inner_content_slice)`.
    /// Perfect for rendering the final "Footnotes" section at the bottom of a Markdown page.
    pub fn definitions_iter(
        &self,
    ) -> impl Iterator<Item = (&'tokens str, &'tokens [(Token<'input>, Range<usize>)])> {
        self.definitions
            .iter()
            .map(|(&label, slice)| (label, Self::strip_definition_tags(slice)))
    }

    /// Checks if a footnote reference has a valid definition (is not an orphan).
    #[inline]
    pub fn has_definition(&self, label: &str) -> bool {
        self.definitions.contains_key(label)
    }

    /// Returns the total number of references made to a given footnote label in the text.
    ///
    /// Useful for determining back-reference suffix counts or identifying
    /// unreferenced/orphan definitions that shouldn't be rendered in the footer.
    #[inline]
    pub fn reference_count(&self, label: &str) -> usize {
        self.references.get(label).map_or(0, |v| v.len())
    }

    /// Checks if a given footnote label has been referenced at least once.
    #[inline]
    pub fn is_referenced(&self, label: &str) -> bool {
        self.references.contains_key(label)
    }

    /// Finds the 0-based index of a specific reference token among all references sharing the same label.
    ///
    /// If `[^1]` appears three times in the document, calling this for the second occurrence
    /// will return `Some(1)`. This is essential for generating unique HTML back-link IDs,
    /// such as `id="fnref:my_label:2"`.
    ///
    /// Uses pointer equality via `core::ptr::eq` for $O(k)$ lookup speed where $k$ is the number
    /// of references for this specific label.
    pub fn reference_index_of(
        &self,
        label: &str,
        target_token_pair: &(Token<'input>, Range<usize>),
    ) -> Option<usize> {
        self.references
            .get(label)?
            .iter()
            // Compare memory addresses of the fat pointers from the original tokens slice
            // to uniquely identify this specific reference instance in the DOM/AST.
            .position(|&stored_pair| core::ptr::eq(stored_pair, target_token_pair))
    }

    /// Returns an iterator over all unique footnote labels that have been referenced in the text.
    ///
    /// Can be used for document validation or statistical collection.
    pub fn referenced_labels(&self) -> impl Iterator<Item = &'_ &'tokens str> + '_ {
        self.references.keys()
    }
}

/// Extension trait for building a [`FootnoteIndex`] from a token slice.
///
/// Implemented for `[(Token<'input>, Range<usize>)]`, which also covers
/// `Vec<(Token<'input>, Range<usize>)>` via `Deref` coercion.
pub trait FootnoteIndexExt<'input> {
    /// Builds an index for fast bidirectional lookup between footnote
    /// references and definitions.
    fn footnote_index<'this>(&'this self) -> FootnoteIndex<'this, 'input>
    where
        'this: 'input;
}

impl<'input> FootnoteIndexExt<'input> for [(Token<'input>, Range<usize>)] {
    fn footnote_index<'this>(&'this self) -> FootnoteIndex<'this, 'input>
    where
        'this: 'input,
    {
        FootnoteIndex::build(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{InterceptorEnum, TokenStreamBuilder};

    macro_rules! assert_footnote_snapshot {
        ($index:expr, $label:expr) => {{
            let slice: Vec<_> = $index
                .full_slice_by_label($label)
                .expect(concat!("Footnote with label '", $label, "' not found"))
                .iter()
                .map(|(token, range)| (token, ::core::ops::Range::from(*range)))
                .collect();

            ::insta::assert_json_snapshot!(concat!("footnote_", $label), slice);
        }};
    }

    fn make_token_stream(source: &str) -> Vec<(Token<'_>, Range<usize>)> {
        TokenStreamBuilder::<InterceptorEnum>::new()
            .old_footnotes(true) // CRITICAL: pulldown-cmark ignores footnotes without this
            .build(source)
            .collect()
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn basic_index_build_and_lookup() {
        let source = "Text with a reference[^1].\n\n[^1]: This is the definition.";
        let tokens = make_token_stream(source);
        let index = FootnoteIndex::build(&tokens);

        assert!(index.has_definition("1"));
        assert!(index.is_referenced("1"));
        assert_eq!(index.reference_count("1"), 1);

        let meta = index.definition_meta("1").unwrap();
        assert_eq!(meta.label(), "1");

        assert_footnote_snapshot!(index, "1");
        //assert_eq!(
        //    &index.full_slice_by_label("1").unwrap()[1..2],
        //    index.content_by_label("1").unwrap()
        //);
    }
}
