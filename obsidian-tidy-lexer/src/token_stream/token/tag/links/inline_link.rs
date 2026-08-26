use alloc::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InlineLink<'input> {
    pub(crate) destination: Cow<'input, str>,
    pub(crate) title: Option<Cow<'input, str>>,
}

impl<'input> InlineLink<'input> {
    pub fn destination(&self) -> &str {
        self.destination.as_ref()
    }

    pub fn title(&self) -> Option<&str> {
        self.title.as_ref().map(Cow::as_ref)
    }
}

crate::__private::impl_as_target_self!(InlineLink<'_>);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::{InterceptorEnum, Tag, Token, TokenStreamBuilder, TracingTokenStreamExt};
    use core::range::Range;
    use proptest::prelude::*;

    fn make_token_stream(source: &str) -> impl Iterator<Item = (Token<'_>, Range<usize>)> {
        TokenStreamBuilder::<InterceptorEnum>::new()
            .build(source)
            .with_tracing()
    }

    fn collect_inline_links(source: &str) -> Vec<InlineLink<'_>> {
        make_token_stream(source)
            .filter_map(|(token, _)| token.into_start().and_then(|tag| tag.into_inline_link()))
            .collect()
    }

    fn count_start_inline_links(source: &str) -> usize {
        make_token_stream(source)
            .filter(|(token, _)| token.as_start().is_some_and(|tag| tag.is_inline_link()))
            .count()
    }

    fn count_end_inline_links(source: &str) -> usize {
        make_token_stream(source)
            .filter(|(token, _)| token.as_end().is_some_and(|tag_end| tag_end.is_link()))
            .count()
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn basic_inline_link() {
        let source = "[click here](https://example.com)";
        let links = collect_inline_links(source);

        assert_eq!(links.len(), 1);
        assert_eq!(links[0].destination(), "https://example.com");
        assert_eq!(links[0].title(), None);
        assert_eq!(
            count_start_inline_links(source),
            count_end_inline_links(source)
        );
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn inline_link_with_double_quote_title() {
        let source = "[click here](https://example.com \"Example Title\")";
        let links = collect_inline_links(source);

        assert_eq!(links.len(), 1);
        assert_eq!(links[0].destination(), "https://example.com");
        assert_eq!(links[0].title(), Some("Example Title"));
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn inline_link_with_single_quote_title() {
        let source = "[click here](https://example.com 'Example Title')";
        let links = collect_inline_links(source);

        assert_eq!(links.len(), 1);
        assert_eq!(links[0].destination(), "https://example.com");
        assert_eq!(links[0].title(), Some("Example Title"));
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn inline_link_with_parentheses_title() {
        let source = "[click here](https://example.com (Example Title))";
        let links = collect_inline_links(source);

        assert_eq!(links.len(), 1);
        assert_eq!(links[0].destination(), "https://example.com");
        assert_eq!(links[0].title(), Some("Example Title"));
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn inline_link_with_many_space() {
        let source = "[click here](https://example.com    'Example Title')";
        let links = collect_inline_links(source);

        assert_eq!(links.len(), 1);
        assert_eq!(links[0].destination(), "https://example.com");
        assert_eq!(links[0].title(), Some("Example Title"));
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn inline_link_empty_parts() {
        let source = "[](https://example.com) and [text]()";
        let links = collect_inline_links(source);

        assert_eq!(links.len(), 2);
        assert_eq!(links[0].destination(), "https://example.com");
        assert_eq!(links[0].title(), None);

        assert_eq!(links[1].destination(), "");
        assert_eq!(links[1].title(), None);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn inline_link_complex_url() {
        let source = "[docs](https://docs.rs/pulldown-cmark/latest/pulldown_cmark/enum.LinkType.html#variant.Inline?query=1#fragment)";
        let links = collect_inline_links(source);

        assert_eq!(links.len(), 1);
        assert_eq!(links[0].title(), None);
        assert_eq!(
            links[0].destination(),
            "https://docs.rs/pulldown-cmark/latest/pulldown_cmark/enum.LinkType.html#variant.Inline?query=1#fragment"
        );
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn inline_link_with_nested_formatting() {
        // It is work in Obsidian
        let source = "[**bold** and *italic* text](https://example.com)";
        let links = collect_inline_links(source);

        assert_eq!(links.len(), 1);
        assert_eq!(links[0].title(), None);
        assert_eq!(links[0].destination(), "https://example.com");

        let tokens: Vec<_> = make_token_stream(source).collect();

        let has_strong = tokens
            .iter()
            .any(|(token, _)| matches!(token, Token::Start(Tag::Strong)));

        let has_emphasis = tokens
            .iter()
            .any(|(token, _)| matches!(token, Token::Start(Tag::Emphasis)));

        assert!(has_strong, "Should parse strong inside link");
        assert!(has_emphasis, "Should parse emphasis inside link");
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn inline_link_balanced_tags_multiple() {
        let source = "[a](b) some text [c](d \"title\")";

        assert_eq!(count_start_inline_links(source), 2);
        assert_eq!(count_end_inline_links(source), 2);
    }

    #[test]
    #[cfg_attr(not(miri), tracing_test::traced_test)]
    fn inline_link_snapshot() {
        let source = "[**bold** link](https://example.com \"title\")";
        let tokens: Vec<_> = make_token_stream(source)
            .map(|(token, range)| (token, core::ops::Range::from(range)))
            .collect();

        #[cfg(not(miri))]
        insta::assert_json_snapshot!(tokens);
    }

    #[test]
    fn no_inline_links_in_plain_text() {
        let source = "Just some plain text with (parentheses) and [brackets].";

        assert_eq!(count_start_inline_links(source), 0);
        assert_eq!(count_end_inline_links(source), 0);
    }

    fn simple_url_strategy() -> impl Strategy<Value = String> {
        (
            // 1. Choose the protocol scheme
            prop_oneof!["http", "https"],
            // 2. Optional subdomain (30% probability of being generated)
            prop::option::weighted(0.3, "[a-z0-9]{2,5}"),
            // 3. Primary domain name
            "[a-z0-9]{3,10}",
            // 4. Top-Level Domain (TLD) zone
            prop_oneof!["com", "org", "net", "io", "ru", "dev"],
            // 5. A vector of path segments (from 0 to 3 elements)
            prop::collection::vec("[a-z0-9]{2,8}", 0..4),
        )
            .prop_map(|(scheme, maybe_sub, domain, tld, path_segments)| {
                // Construct the host part, accounting for the optional subdomain
                let host = match maybe_sub {
                    Some(sub) => format!("{}.{}.{}", sub, domain, tld),
                    None => format!("{}.{}", domain, tld),
                };

                // Build the path component (ensure it starts with a slash)
                let path = if path_segments.is_empty() {
                    String::from("/")
                } else {
                    format!("/{}", path_segments.join("/"))
                };

                format!("{}://{}{}", scheme, host, path)
            })
    }

    fn complex_url_strategy() -> impl Strategy<Value = String> {
        (
            // Supported URL schemes
            prop_oneof!["http", "https", "ftp", "ws", "wss"],
            // Optional (user, password) tuple
            prop::option::of(("[a-z0-9]{3,6}", "[a-zA-Z0-9]{4,8}")),
            // Main domain name
            "[a-z0-9]{3,12}",
            // Top-Level Domains (TLD)
            prop_oneof![
                Just("com".to_string()),
                Just("org".to_string()),
                Just("net".to_string()),
                Just("gov".to_string()),
                Just("co.uk".to_string())
            ],
            // Optional custom port number
            prop::option::of(1..=65535u16),
            // Vector of path segments
            prop::collection::vec("[a-zA-Z0-9_-]{1,8}", 0..5),
            // Query parameters as a vector of (key, value) pairs (0 to 3 pairs)
            prop::collection::vec(("[a-z]{2,5}", "[a-zA-Z0-9%]{1,10}"), 0..4),
            // Optional fragment/hash anchor (#anchor)
            prop::option::of("[a-zA-Z0-9_-]{3,10}"),
        )
            .prop_map(
                |(scheme, auth, domain, tld, port, path_vec, query_vec, fragment)| {
                    let mut url = format!("{}://", scheme);

                    // 1. Append user credentials if present
                    if let Some((user, pass)) = auth {
                        url.push_str(&format!("{}:{}@", user, pass));
                    }

                    // 2. Append host (domain + TLD) and the optional port
                    url.push_str(&format!("{}.{}", domain, tld));
                    if let Some(p) = port {
                        url.push_str(&format!(":{}", p));
                    }

                    // 3. Append the hierarchical path component
                    if path_vec.is_empty() {
                        url.push('/');
                    } else {
                        url.push_str(&format!("/{}", path_vec.join("/")));
                    }

                    // 4. Build and append the query string (?key=value&key2=value2)
                    if !query_vec.is_empty() {
                        let pairs: Vec<String> = query_vec
                            .into_iter()
                            .map(|(k, v)| format!("{}={}", k, v))
                            .collect();
                        url.push_str(&format!("?{}", pairs.join("&")));
                    }

                    // 5. Append the fragment anchor if present
                    if let Some(f) = fragment {
                        url.push_str(&format!("#{}", f));
                    }

                    url
                },
            )
    }

    proptest! {
        #[test]
        #[cfg_attr(miri, ignore)]
        fn prop_inline_link_basic(url in simple_url_strategy(), text in r"[a-zA-Z0-9]{1,20}") {
            let source = format!("[{text}]({url})");
            let links = collect_inline_links(&source);

            prop_assert_eq!(links.len(), 1, "Should parse exactly one link: source = `{}`", source);
            prop_assert_eq!(links[0].destination(), url.as_str(), "Destination mismatch: source = `{}`", source);
            prop_assert_eq!(links[0].title(), None, "Title should be None: source = `{}`", source);

            prop_assert_eq!(count_start_inline_links(&source), 1);
            prop_assert_eq!(count_end_inline_links(&source), 1);
        }

        #[test]
        #[cfg_attr(miri, ignore)]
        fn prop_inline_link_with_complex_url(url in complex_url_strategy(), text in r"[a-zA-Z0-9]{1,20}") {
            let source = format!("[{text}]({url})");
            let links = collect_inline_links(&source);

            prop_assert_eq!(links.len(), 1, "Should parse exactly one link: source = `{}`", source);
            prop_assert_eq!(links[0].destination(), url.as_str(), "Destination mismatch: source = `{}`", source);
            prop_assert_eq!(links[0].title(), None, "Title should be None: source = `{}`", source);

            prop_assert_eq!(count_start_inline_links(&source), 1);
            prop_assert_eq!(count_end_inline_links(&source), 1);
        }

        #[test]
        #[cfg_attr(miri, ignore)]
        fn prop_inline_link_with_title(
            url in simple_url_strategy(),
            text in r"[a-zA-Z0-9 ]{1,10}",
            space in r" {1,15}",
            title in r#"[\p{L}\p{N} ]{1,15}"#
        ) {
            let source = format!("[{text}]({url}{space}\"{title}\")");
            let links = collect_inline_links(&source);

            prop_assert_eq!(links.len(), 1, "Should parse exactly one link: source = `{}`", source);
            prop_assert_eq!(links[0].destination(), url.as_str(), "Destination mismatch: source = `{}`", source);
            prop_assert_eq!(links[0].title(), Some(title.as_str()), "Title should be Some: source = `{}`", source);

            prop_assert_eq!(count_start_inline_links(&source), 1);
            prop_assert_eq!(count_end_inline_links(&source), 1);
        }

        #[test]
        #[cfg_attr(miri, ignore)]
        fn prop_inline_link_with_title_and_complex_url(
            url in complex_url_strategy(),
            text in r"[a-zA-Z0-9 ]{1,10}",
            space in r" {1,15}",
            title in r#"[\p{L}\p{N} ]{1,15}"#
        ) {
            let source = format!("[{text}]({url}{space}\"{title}\")");
            let links = collect_inline_links(&source);

            prop_assert_eq!(links.len(), 1, "Should parse exactly one link: source = `{}`", source);
            prop_assert_eq!(links[0].destination(), url.as_str(), "Destination mismatch: source = `{}`", source);
            prop_assert_eq!(links[0].title(), Some(title.as_str()), "Title should be Some: source = `{}`", source);

            prop_assert_eq!(count_start_inline_links(&source), 1);
            prop_assert_eq!(count_end_inline_links(&source), 1);
        }
    }
}
