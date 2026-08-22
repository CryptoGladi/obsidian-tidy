use pulldown_cmark::HeadingLevel as MarkHeadingLevel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum HeadingLevel {
    H1 = 1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl From<MarkHeadingLevel> for HeadingLevel {
    fn from(level: MarkHeadingLevel) -> Self {
        match level {
            MarkHeadingLevel::H1 => HeadingLevel::H1,
            MarkHeadingLevel::H2 => HeadingLevel::H2,
            MarkHeadingLevel::H3 => HeadingLevel::H3,
            MarkHeadingLevel::H4 => HeadingLevel::H4,
            MarkHeadingLevel::H5 => HeadingLevel::H5,
            MarkHeadingLevel::H6 => HeadingLevel::H6,
        }
    }
}

impl core::fmt::Display for HeadingLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "H{}", *self as u8)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Heading {
    level: HeadingLevel,
}

impl Heading {
    #[must_use]
    pub const fn new(level: HeadingLevel) -> Self {
        Self { level }
    }

    #[must_use]
    pub const fn level(&self) -> HeadingLevel {
        self.level
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{InterceptorEnum, Token, TokenStreamBuilder, TracingTokenStreamExt};
    use core::range::Range;

    #[test]
    fn display_all_variants() {
        let cases = [
            (HeadingLevel::H1, "H1"),
            (HeadingLevel::H2, "H2"),
            (HeadingLevel::H3, "H3"),
            (HeadingLevel::H4, "H4"),
            (HeadingLevel::H5, "H5"),
            (HeadingLevel::H6, "H6"),
        ];

        for (level, expected) in cases {
            assert_eq!(
                level.to_string(),
                expected,
                "Failed formatting for {:?}",
                level
            );
        }
    }

    #[test]
    fn from_pulldown_cmark_heading_level() {
        let cases = [
            (MarkHeadingLevel::H1, HeadingLevel::H1),
            (MarkHeadingLevel::H2, HeadingLevel::H2),
            (MarkHeadingLevel::H3, HeadingLevel::H3),
            (MarkHeadingLevel::H4, HeadingLevel::H4),
            (MarkHeadingLevel::H5, HeadingLevel::H5),
            (MarkHeadingLevel::H6, HeadingLevel::H6),
        ];

        for (level, expected) in cases {
            assert_eq!(HeadingLevel::from(level), expected);
        }
    }

    fn make_token_stream<'input>(
        source: &'input str,
    ) -> impl Iterator<Item = (Token<'input>, Range<usize>)> {
        TokenStreamBuilder::<InterceptorEnum>::new()
            .build(source)
            .with_tracing()
    }

    fn collect_tokens<'input>(source: &'input str) -> Vec<(Token<'input>, Range<usize>)> {
        let lexer = make_token_stream(source);
        lexer.into_iter().collect()
    }

    fn collect_headings(source: &str) -> Vec<Heading> {
        let tokens = collect_tokens(source);

        tokens
            .into_iter()
            .filter_map(|(token, _)| token.into_start().and_then(|tag| tag.into_heading()))
            .collect()
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn check_all_heading_level() {
        let cases = [
            ("# H1", HeadingLevel::H1),
            ("## H2", HeadingLevel::H2),
            ("### H3", HeadingLevel::H3),
            ("#### H4", HeadingLevel::H4),
            ("##### H5", HeadingLevel::H5),
            ("###### H6", HeadingLevel::H6),
        ];

        for (source, expected) in cases {
            let headings = collect_headings(source);

            assert_eq!(headings.len(), 1);
            assert_eq!(headings[0].level(), expected);
        }
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn invalid_heading() {
        let source = "####### H7";

        let headings = collect_headings(source);
        assert!(headings.is_empty());
    }
}
