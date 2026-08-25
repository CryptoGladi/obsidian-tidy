use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Alignment {
    None,
    Left,
    Center,
    Right,
}

static_assertions::assert_impl_all!(Alignment: Copy, Clone);

impl From<pulldown_cmark::Alignment> for Alignment {
    fn from(alignment: pulldown_cmark::Alignment) -> Self {
        use pulldown_cmark::Alignment as MarkAlignment;

        match alignment {
            MarkAlignment::None => Self::None,
            MarkAlignment::Left => Self::Left,
            MarkAlignment::Center => Self::Center,
            MarkAlignment::Right => Self::Right,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Table {
    alignments: Vec<Alignment>,
}

impl Table {
    pub fn new<T>(alignments: T) -> Self
    where
        T: IntoIterator<Item = Alignment>,
    {
        Self {
            alignments: alignments.into_iter().collect(),
        }
    }
}

impl<T> From<T> for Table
where
    T: IntoIterator<Item = pulldown_cmark::Alignment>,
{
    fn from(value: T) -> Self {
        let alignments = value.into_iter().map(Alignment::from);
        Table::new(alignments)
    }
}

crate::__private::impl_as_target_self!(Table);

#[cfg(test)]
mod tests {
    use crate::{InterceptorEnum, Token, TokenStreamBuilder, TracingTokenStreamExt};
    use core::range::Range;

    macro_rules! assert_json_snapshot {
        ($tokens:ident) => {{
            let tokens: Vec<_> = $tokens
                .into_iter()
                .map(|(token, range)| (token, std::ops::Range::from(range)))
                .collect();

            insta::assert_json_snapshot!(tokens);
        }};
    }

    fn token_stream(source: &str) -> Vec<(Token<'_>, Range<usize>)> {
        TokenStreamBuilder::<InterceptorEnum>::new()
            .build(source)
            .with_tracing()
            .collect()
    }

    fn find_table<'input>(
        tokens: &'input [(Token<'input>, Range<usize>)],
    ) -> Option<&'input crate::Table> {
        tokens
            .iter()
            .find_map(|(token, _)| token.as_start().and_then(|tag| tag.as_table()))
    }

    // === Basic Table Tests ===

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn simple_table_no_alignment() {
        let source = "| Header 1 | Header 2 |
|----------|----------|
| Cell 1   | Cell 2   |";
        let tokens = token_stream(source);

        let table = find_table(&tokens).expect("should find table");
        assert_eq!(table.alignments.len(), 2);
        assert_eq!(table.alignments[0], crate::Alignment::None);
        assert_eq!(table.alignments[1], crate::Alignment::None);

        #[cfg(not(miri))]
        assert_json_snapshot!(tokens);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn table_with_left_alignment() {
        let source = "| Left |
|:-----|
| L    |";
        let tokens = token_stream(source);

        let table = find_table(&tokens).expect("should find table");
        assert_eq!(table.alignments.len(), 1);
        assert_eq!(table.alignments[0], crate::Alignment::Left);

        #[cfg(not(miri))]
        assert_json_snapshot!(tokens);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn table_with_center_alignment() {
        let source = "| Center |
|:------:|
| C      |";
        let tokens = token_stream(source);

        let table = find_table(&tokens).expect("should find table");
        assert_eq!(table.alignments.len(), 1);
        assert_eq!(table.alignments[0], crate::Alignment::Center);

        #[cfg(not(miri))]
        assert_json_snapshot!(tokens);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn table_with_right_alignment() {
        let source = "| Right |
|------:|
| R     |";
        let tokens = token_stream(source);

        let table = find_table(&tokens).expect("should find table");
        assert_eq!(table.alignments.len(), 1);
        assert_eq!(table.alignments[0], crate::Alignment::Right);

        #[cfg(not(miri))]
        assert_json_snapshot!(tokens);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn table_with_mixed_alignments() {
        let source = "| Left | Center | Right | None |
|:-----|:------:|------:|------|
| L    | C      | R     | N    |";
        let tokens = token_stream(source);

        let table = find_table(&tokens).expect("should find table");
        assert_eq!(table.alignments.len(), 4);
        assert_eq!(table.alignments[0], crate::Alignment::Left);
        assert_eq!(table.alignments[1], crate::Alignment::Center);
        assert_eq!(table.alignments[2], crate::Alignment::Right);
        assert_eq!(table.alignments[3], crate::Alignment::None);

        #[cfg(not(miri))]
        assert_json_snapshot!(tokens);
    }

    // === Multiple Rows Tests ===

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn table_with_multiple_rows() {
        let source = "| H1 | H2 |
|----|----|
| R1C1 | R1C2 |
| R2C1 | R2C2 |
| R3C1 | R3C2 |";
        let tokens = token_stream(source);

        let table = find_table(&tokens).expect("should find table");
        assert_eq!(table.alignments.len(), 2);

        #[cfg(not(miri))]
        assert_json_snapshot!(tokens);
    }

    // === Edge Cases ===

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn table_with_single_column() {
        let source = "| Single |
|--------|
| Cell   |";
        let tokens = token_stream(source);

        let table = find_table(&tokens).expect("should find table");
        assert_eq!(table.alignments.len(), 1);
        assert_eq!(table.alignments[0], crate::Alignment::None);

        #[cfg(not(miri))]
        assert_json_snapshot!(tokens);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn table_with_many_columns() {
        let source = "| C1 | C2 | C3 | C4 | C5 |
|----|----|----|----|----|
| 1  | 2  | 3  | 4  | 5  |";
        let tokens = token_stream(source);

        let table = find_table(&tokens).expect("should find table");
        assert_eq!(table.alignments.len(), 5);

        #[cfg(not(miri))]
        assert_json_snapshot!(tokens);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn table_with_empty_cells() {
        let source = "| H1 | H2 | H3 |
|----|----|----|
|    | X  |    |
| Y  |    | Z  |";
        let tokens = token_stream(source);

        let table = find_table(&tokens).expect("should find table");
        assert_eq!(table.alignments.len(), 3);

        #[cfg(not(miri))]
        assert_json_snapshot!(tokens);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn table_with_formatting_in_cells() {
        let source = "| Header |
|--------|
| **bold** |
| *italic* |
| `code` |";
        let tokens = token_stream(source);

        let table = find_table(&tokens).expect("should find table");
        assert_eq!(table.alignments.len(), 1);

        #[cfg(not(miri))]
        assert_json_snapshot!(tokens);
    }

    // === Invalid Tables ===

    #[test]
    fn not_a_table_without_separator() {
        let source = "| Header 1 | Header 2 |
| Cell 1   | Cell 2   |";
        let tokens = token_stream(source);

        // Should not be parsed as table without separator line
        assert!(find_table(&tokens).is_none());
    }

    #[test]
    fn not_a_table_in_plain_text() {
        let source = "This is just text with | pipes | in it.";
        let tokens = token_stream(source);

        assert!(find_table(&tokens).is_none());
    }

    // === Alignment Variations ===

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn table_with_minimal_separator() {
        let source = "| H |
|-|
| C |";
        let tokens = token_stream(source);

        let table = find_table(&tokens).expect("should find table");
        assert_eq!(table.alignments.len(), 1);

        #[cfg(not(miri))]
        assert_json_snapshot!(tokens);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn table_with_extra_spaces_in_separator() {
        let source = "| H1 | H2 |
| :--- | :---: |
| C1   | C2   |";
        let tokens = token_stream(source);

        let table = find_table(&tokens).expect("should find table");
        assert_eq!(table.alignments.len(), 2);
        assert_eq!(table.alignments[0], crate::Alignment::Left);
        assert_eq!(table.alignments[1], crate::Alignment::Center);

        #[cfg(not(miri))]
        assert_json_snapshot!(tokens);
    }

    // === Integration Tests ===

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn table_balanced_tags() {
        let source = "| H1 | H2 |
|----|----|
| C1 | C2 |";
        let tokens = token_stream(source);

        let start_count = tokens
            .iter()
            .filter(|(t, _)| t.as_start().is_some_and(|tag| tag.is_table()))
            .count();

        let end_count = tokens
            .iter()
            .filter(|(t, _)| t.as_end().is_some_and(|tag_end| tag_end.is_table()))
            .count();

        assert_eq!(start_count, end_count, "Table tags should be balanced");
        assert_eq!(start_count, 1);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn multiple_tables_in_document() {
        let source = "# Document

| Table 1 |
|---------|
| Cell 1  |

Some text

| Table 2 |
|---------|
| Cell 2  |";
        let tokens = token_stream(source);

        let table_count = tokens
            .iter()
            .filter(|(t, _)| t.as_start().is_some_and(|tag| tag.is_table()))
            .count();

        assert_eq!(table_count, 2, "should find 2 tables");

        #[cfg(not(miri))]
        assert_json_snapshot!(tokens);
    }

    /* TODO
    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn table_with_links_and_images() {
        let source = "| Header |
|--------|
| [link](url) |
| ![img](url) |";
        let tokens = token_stream(source);

        let table = find_table(&tokens).expect("should find table");
        assert_eq!(table.alignments.len(), 1);

        #[cfg(not(miri))]
        assert_json_snapshot!(tokens);
    } */
}
