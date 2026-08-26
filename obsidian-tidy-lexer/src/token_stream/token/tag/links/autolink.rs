use alloc::borrow::Cow;
use derive_more::IsVariant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, IsVariant)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AutolinkKind {
    /// URI (http, https, ftp, file, git...)
    Uri,

    Email,
}

static_assertions::assert_impl_all!(AutolinkKind: Copy, Clone);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Autolink<'input> {
    pub(crate) url: Cow<'input, str>,
    pub(crate) kind: AutolinkKind,
}

impl<'input> Autolink<'input> {
    pub fn url(&self) -> &str {
        self.url.as_ref()
    }

    #[inline]
    pub const fn kind(&self) -> AutolinkKind {
        self.kind
    }

    pub fn is_uri(&self) -> bool {
        self.kind().is_uri()
    }

    pub fn is_email(&self) -> bool {
        self.kind().is_email()
    }
}

crate::__private::impl_as_target_self!(Autolink<'_>);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{InterceptorEnum, Token, TokenStreamBuilder, TracingTokenStreamExt};
    use core::range::Range;
    use proptest::prelude::*;

    fn make_token_stream(source: &str) -> impl Iterator<Item = (Token<'_>, Range<usize>)> {
        TokenStreamBuilder::<InterceptorEnum>::new()
            .build(source)
            .with_tracing()
    }

    fn collect_autolinks(source: &str) -> Vec<Autolink<'_>> {
        make_token_stream(source)
            .filter_map(|(token, _)| token.into_start().and_then(|tag| tag.into_autolink()))
            .collect()
    }

    fn count_start_autolinks(source: &str) -> usize {
        make_token_stream(source)
            .filter(|(token, _)| token.as_start().is_some_and(|tag| tag.is_autolink()))
            .count()
    }

    fn count_end_autolinks(source: &str) -> usize {
        make_token_stream(source)
            .filter(|(token, _)| token.as_end().is_some_and(|tag_end| tag_end.is_link()))
            .count()
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn basic_url_autolink() {
        let source = "<https://example.com>";
        let links = collect_autolinks(source);

        assert_eq!(links.len(), 1);
        assert_eq!(links[0].url(), "https://example.com");
        assert!(links[0].is_uri());

        assert_eq!(count_start_autolinks(source), count_end_autolinks(source));
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn basic_email_autolink() {
        let source = "<foo@bar.com>";
        let links = collect_autolinks(source);

        assert_eq!(links.len(), 1);
        assert_eq!(links[0].url(), "foo@bar.com");
        assert!(links[0].is_email());

        assert_eq!(count_start_autolinks(source), count_end_autolinks(source));
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn complex_url_autolink() {
        let source = "<ftp://user:pass@sub.example.com:8080/path?query=1#frag>";
        let links = collect_autolinks(source);

        assert_eq!(links.len(), 1);
        assert_eq!(
            links[0].url(),
            "ftp://user:pass@sub.example.com:8080/path?query=1#frag"
        );

        assert!(!links[0].is_email());
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn multiple_autolinks_mixed() {
        let source = "Check <https://a.com> and email <admin@b.com> or <http://c.com>";
        let links = collect_autolinks(source);

        assert_eq!(links.len(), 3);
        assert!(links[0].is_uri());
        assert!(links[1].is_email());
        assert!(links[2].is_uri());

        assert_eq!(count_start_autolinks(source), 3);
        assert_eq!(count_end_autolinks(source), 3);
    }

    #[test]
    fn not_an_autolink_plain_brackets() {
        let source = "Just some <text> with brackets";
        let links = collect_autolinks(source);

        assert_eq!(links.len(), 0);
        assert_eq!(count_start_autolinks(source), 0);
        assert_eq!(count_end_autolinks(source), 0);
    }

    #[test]
    fn not_an_autolink_invalid_email() {
        let source = "This is not an email: <notanemail>";
        let links = collect_autolinks(source);

        assert_eq!(links.len(), 0);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn autolink_snapshot() {
        let source = "Visit <https://rust-lang.org> or email <admin@rust-lang.org>";
        let tokens: Vec<_> = make_token_stream(source)
            .map(|(token, range)| (token, core::ops::Range::from(range)))
            .collect();

        #[cfg(not(miri))]
        insta::assert_json_snapshot!(tokens);
    }

    fn email_strategy() -> impl Strategy<Value = String> {
        (
            // local part
            "[a-zA-Z0-9._%+-]{3,10}",
            // domain
            prop_oneof![
                Just("gmail.com"),
                Just("yahoo.com"),
                Just("example.org"),
                Just("test.io")
            ],
        )
            .prop_map(|(local, domain)| format!("{}@{}", local, domain))
    }

    fn url_strategy() -> impl Strategy<Value = String> {
        (
            prop_oneof!["http", "https", "ftp"],
            "[a-zA-Z0-9-]{3,10}", // domain
            prop_oneof!["com", "org", "net", "io"],
            prop::collection::vec("[a-zA-Z0-9_-]{1,5}", 0..3), // path segments
        )
            .prop_map(|(scheme, domain, tld, paths)| {
                let path = if paths.is_empty() {
                    String::new()
                } else {
                    format!("/{}", paths.join("/"))
                };
                format!("{}://{}.{}{}", scheme, domain, tld, path)
            })
    }

    proptest! {
        #[test]
        #[cfg_attr(miri, ignore)]
        fn prop_basic_email_autolink(email in email_strategy()) {
            let source = format!("<{}>", email);
            let links = collect_autolinks(&source);

            prop_assert_eq!(links.len(), 1, "Should parse exactly one autolink: source = `{}`", source);
            prop_assert_eq!(links[0].url(), email.as_str(), "URL mismatch: source = `{}`", source);
            prop_assert!(links[0].is_email(), "Should be detected as email: source = `{}`", source);

            prop_assert_eq!(count_start_autolinks(&source), 1);
            prop_assert_eq!(count_end_autolinks(&source), 1);
        }

        #[test]
        #[cfg_attr(miri, ignore)]
        fn prop_mixed_autolinks_in_text(
            email in email_strategy(),
            url in url_strategy(),

            // The text must start with a visible character (letter or digit).
            // According to the CommonMark specification, a line starting with 4 or more spaces
            // is parsed as an Indented Code Block, which completely ignores autolink syntax inside.
            // This constraint prevents false test failures in proptest caused by valid but
            // contextually inapplicable Markdown.
            // Spec: <https://spec.commonmark.org/0.31.2/#indented-code-blocks>
            text in r"[a-zA-Z0-9][a-zA-Z0-9 ]{0,15}"
        ) {
            let source = format!("{} <{}> and {} <{}>", text, email, text, url);
            let links = collect_autolinks(&source);

            prop_assert_eq!(links.len(), 2, "Should parse exactly two autolinks: source = `{}`", source);
            prop_assert!(links[0].is_email(), "First should be email");
            prop_assert!(!links[1].is_email(), "Second should be URL");

            prop_assert_eq!(count_start_autolinks(&source), 2);
            prop_assert_eq!(count_end_autolinks(&source), 2);
        }
    }
}
