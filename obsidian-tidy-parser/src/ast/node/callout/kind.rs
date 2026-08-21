use alloc::borrow::Cow;
use derive_more::IsVariant;
use serde::{Deserialize, Serialize};

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

// Не могу std::str::FromStr из-за lifetime
impl<'ast> From<&'ast str> for CalloutKind<'ast> {
    fn from(raw: &'ast str) -> Self {
        CalloutKind::from(Cow::<'ast, str>::Borrowed(raw))
    }
}

impl<'ast> From<Cow<'ast, str>> for CalloutKind<'ast> {
    fn from(raw: Cow<'ast, str>) -> Self {
        // Note: ASCII-only for performance. Unicode kinds go into Other.
        let lower = raw.to_ascii_lowercase();

        match lower.as_str() {
            "note" => Self::Note,
            "abstract" | "summary" | "tldr" => Self::Abstract,
            "info" => Self::Info,
            "todo" => Self::Todo,
            "tip" | "hint" | "important" => Self::Tip,
            "success" | "check" | "done" => Self::Success,
            "question" | "help" | "faq" => Self::Question,
            "warning" | "caution" | "attention" => Self::Warning,
            "failure" | "fail" | "missing" => Self::Failure,
            "danger" | "error" => Self::Danger,
            "bug" => Self::Bug,
            "example" => Self::Example,
            "quote" | "cite" => Self::Quote,
            _ => Self::Other(raw),
        }
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
