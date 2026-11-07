use pulldown_cmark::HeadingLevel as MarkHeadingLevel;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HeadingLevel {
    H1 = 1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

static_assertions::assert_impl_all!(HeadingLevel: Copy, Clone);

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Heading {
    level: HeadingLevel,
}

crate::__private::impl_as_target_self!(Heading);

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
                "Failed formatting for {level:?}"
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

    fn make_token_stream(source: &str) -> impl Iterator<Item = (Token<'_>, Range<usize>)> {
        TokenStreamBuilder::<InterceptorEnum>::new()
            .build(source)
            .with_tracing()
    }

    fn collect_tokens(source: &str) -> Vec<(Token<'_>, Range<usize>)> {
        let lexer = make_token_stream(source);
        lexer.into_iter().collect()
    }

    fn collect_headings(source: &str) -> Vec<Heading> {
        let tokens = collect_tokens(source);

        tokens
            .into_iter()
            .filter_map(|(token, _)| token.into_start().and_then(super::super::Tag::into_heading))
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
        assert_eq!(headings, [] as [crate::Heading; 0]);
    }
}
