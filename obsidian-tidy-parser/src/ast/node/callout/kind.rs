use alloc::borrow::Cow;
use derive_more::IsVariant;
use phf::phf_map;
use serde::{Deserialize, Serialize};
use unicase::UniCase;

#[derive(Debug, Clone, Copy)]
enum StaticKind {
    Note,
    Abstract,
    Info,
    Todo,
    Tip,
    Success,
    Question,
    Warning,
    Failure,
    Danger,
    Bug,
    Example,
    Quote,
}

static CALLOUTS: phf::Map<UniCase<&'static str>, StaticKind> = phf_map! {
    UniCase::ascii("note") => StaticKind::Note,
    UniCase::ascii("abstract") | UniCase::ascii("summary") | UniCase::ascii("tldr") => StaticKind::Abstract,
    UniCase::ascii("info") => StaticKind::Info,
    UniCase::ascii("todo") => StaticKind::Todo,
    UniCase::ascii("tip") | UniCase::ascii("hint") | UniCase::ascii("important") => StaticKind::Tip,
    UniCase::ascii("success") | UniCase::ascii("check") | UniCase::ascii("done") => StaticKind::Success,
    UniCase::ascii("question") | UniCase::ascii("help") | UniCase::ascii("faq") => StaticKind::Question,
    UniCase::ascii("warning") | UniCase::ascii("caution") | UniCase::ascii("attention") => StaticKind::Warning,
    UniCase::ascii("failure") | UniCase::ascii("fail") | UniCase::ascii("missing") => StaticKind::Failure,
    UniCase::ascii("danger") | UniCase::ascii("error") => StaticKind::Danger,
    UniCase::ascii("bug") => StaticKind::Bug,
    UniCase::ascii("example") => StaticKind::Example,
    UniCase::ascii("quote") | UniCase::ascii("cite") => StaticKind::Quote
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, IsVariant, strum::Display)]
pub enum CalloutKind<'ast> {
    /// Aliases: `summary`, `tldr`
    Abstract,

    Info,

    Todo,

    /// Aliases: `hint`, `important`
    Tip,

    /// Aliases: `check`, `done`
    Success,

    /// Aliases: `help`, `faq`
    Question,

    /// Aliases: `caution`, `attention`
    Warning,

    /// Aliases: `fail`, `missing`
    Failure,

    /// Aliases: `error`
    Danger,

    Bug,

    Note,

    Example,

    /// Aliases: `cite`
    Quote,

    // И остальные
    Other(Cow<'ast, str>),
}

impl From<StaticKind> for CalloutKind<'_> {
    fn from(static_kind: StaticKind) -> Self {
        match static_kind {
            StaticKind::Note => Self::Note,
            StaticKind::Abstract => Self::Abstract,
            StaticKind::Info => Self::Info,
            StaticKind::Todo => Self::Todo,
            StaticKind::Tip => Self::Tip,
            StaticKind::Success => Self::Success,
            StaticKind::Question => Self::Question,
            StaticKind::Warning => Self::Warning,
            StaticKind::Failure => Self::Failure,
            StaticKind::Danger => Self::Danger,
            StaticKind::Bug => Self::Bug,
            StaticKind::Example => Self::Example,
            StaticKind::Quote => Self::Quote,
        }
    }
}

// Не могу std::str::FromStr из-за lifetime
impl<'ast> From<&'ast str> for CalloutKind<'ast> {
    fn from(raw: &'ast str) -> Self {
        CalloutKind::from(Cow::<'ast, str>::Borrowed(raw))
    }
}

impl<'ast> From<Cow<'ast, str>> for CalloutKind<'ast> {
    fn from(raw: Cow<'ast, str>) -> Self {
        // Note: ASCII-only for performance. Unicode kinds go into Other.
        CALLOUTS.get(&UniCase::ascii(raw.as_ref())).map_or_else(
            || Self::Other(raw),
            |static_callout| Self::from(*static_callout),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn parse_all_known_types() {
        // От регистра не зависим!
        let cases = vec![
            ("noTe", CalloutKind::Note),
            ("aBstract", CalloutKind::Abstract),
            ("suMmary", CalloutKind::Abstract),
            ("tlDr", CalloutKind::Abstract),
            ("inFo", CalloutKind::Info),
            ("todO", CalloutKind::Todo),
            ("tIp", CalloutKind::Tip),
            ("hInt", CalloutKind::Tip),
            ("imPortant", CalloutKind::Tip),
            ("sUccess", CalloutKind::Success),
            ("chEck", CalloutKind::Success),
            ("dOne", CalloutKind::Success),
            ("qUestion", CalloutKind::Question),
            ("heLp", CalloutKind::Question),
            ("faQ", CalloutKind::Question),
            ("Warning", CalloutKind::Warning),
            ("caUtion", CalloutKind::Warning),
            ("attEntion", CalloutKind::Warning),
            ("failUre", CalloutKind::Failure),
            ("faiL", CalloutKind::Failure),
            ("misSing", CalloutKind::Failure),
            ("danGer", CalloutKind::Danger),
            ("erRor", CalloutKind::Danger),
            ("buG", CalloutKind::Bug),
            ("exAmple", CalloutKind::Example),
            ("quOte", CalloutKind::Quote),
            ("ciTe", CalloutKind::Quote),
        ];

        for (input, expected) in cases {
            let result = CalloutKind::from(input);
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn parse_unknown_types() {
        let cases = vec!["custom", "my-type", "xyz", "123", "", " ", "tip-extra"];

        for input in cases {
            let result = CalloutKind::from(input);
            match result {
                CalloutKind::Other(cow) => assert_eq!(cow.as_ref(), input),
                _ => panic!("Expected Other variant for input: {}", input),
            }
        }
    }

    fn any_case_strategy(word: &'static str) -> impl Strategy<Value = String> {
        let regex_pattern = word
            .chars()
            .map(|c| format!("[{}{}]", c.to_lowercase(), c.to_uppercase()))
            .collect::<String>();

        prop::string::string_regex(&regex_pattern)
            .expect("Valid regex generated from static string")
    }

    fn callout_input_strategy<'ast>() -> impl Strategy<Value = (String, CalloutKind<'ast>)> {
        prop_oneof![
            any_case_strategy("note").prop_map(|s| (s, CalloutKind::Note)),
            any_case_strategy("abstract").prop_map(|s| (s, CalloutKind::Abstract)),
            any_case_strategy("summary").prop_map(|s| (s, CalloutKind::Abstract)),
            any_case_strategy("tldr").prop_map(|s| (s, CalloutKind::Abstract)),
            any_case_strategy("info").prop_map(|s| (s, CalloutKind::Info)),
            any_case_strategy("todo").prop_map(|s| (s, CalloutKind::Todo)),
            any_case_strategy("tip").prop_map(|s| (s, CalloutKind::Tip)),
            any_case_strategy("hint").prop_map(|s| (s, CalloutKind::Tip)),
            any_case_strategy("important").prop_map(|s| (s, CalloutKind::Tip)),
            any_case_strategy("success").prop_map(|s| (s, CalloutKind::Success)),
            any_case_strategy("check").prop_map(|s| (s, CalloutKind::Success)),
            any_case_strategy("done").prop_map(|s| (s, CalloutKind::Success)),
            any_case_strategy("question").prop_map(|s| (s, CalloutKind::Question)),
            any_case_strategy("help").prop_map(|s| (s, CalloutKind::Question)),
            any_case_strategy("faq").prop_map(|s| (s, CalloutKind::Question)),
            any_case_strategy("warning").prop_map(|s| (s, CalloutKind::Warning)),
            any_case_strategy("caution").prop_map(|s| (s, CalloutKind::Warning)),
            any_case_strategy("attention").prop_map(|s| (s, CalloutKind::Warning)),
            any_case_strategy("failure").prop_map(|s| (s, CalloutKind::Failure)),
            any_case_strategy("fail").prop_map(|s| (s, CalloutKind::Failure)),
            any_case_strategy("missing").prop_map(|s| (s, CalloutKind::Failure)),
            any_case_strategy("danger").prop_map(|s| (s, CalloutKind::Danger)),
            any_case_strategy("error").prop_map(|s| (s, CalloutKind::Danger)),
            any_case_strategy("bug").prop_map(|s| (s, CalloutKind::Bug)),
            any_case_strategy("example").prop_map(|s| (s, CalloutKind::Example)),
            any_case_strategy("quote").prop_map(|s| (s, CalloutKind::Quote)),
            any_case_strategy("cite").prop_map(|s| (s, CalloutKind::Quote)),
        ]
    }

    proptest! {
        #[test]
        #[cfg_attr(miri, ignore)]
        fn infallible_error_type(s in "[a-z]{1,10}") {
            let s_ref = s.as_str();
            let result = CalloutKind::try_from(s_ref);

            prop_assert!(result.is_ok());
        }

        #[test]
        #[cfg_attr(miri, ignore)]
        fn parse_all_known_types_case_insensitive(
            (input, expected) in callout_input_strategy()
        ) {
            let result = CalloutKind::try_from(input.as_str());

            prop_assert!(result.is_ok(), "Failed to parse input: {}", input);
            prop_assert_eq!(result.unwrap(), expected, "Mismatch for input: {}", input);
        }
    }

    #[test]
    fn empty_string_goes_to_other() {
        let result = CalloutKind::from("");
        assert!(matches!(result, CalloutKind::Other(cow) if cow.is_empty()));
    }

    #[test]
    fn whitespace_goes_to_other() {
        let result = CalloutKind::from(" ");
        assert!(result.is_other());

        let result = CalloutKind::from("\t");
        assert!(result.is_other());
    }

    #[test]
    fn unicode_characters_go_to_other() {
        let result = CalloutKind::from("заметка");
        assert!(result.is_other());

        let result = CalloutKind::from("提示");
        assert!(result.is_other());
    }
}
