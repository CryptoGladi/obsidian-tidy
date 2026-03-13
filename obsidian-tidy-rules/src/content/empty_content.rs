//! Rule for search notes with empty content

use obsidian_parser::note::Note as _;
use obsidian_tidy_core::rule::rule_fabric::GetFabricFromRuleConstMetadata;
use obsidian_tidy_core::rule::violation::{Error as ViolationError, Violation};
use obsidian_tidy_core::rule::{Content, RuleFabric, RuleRunner};
use obsidian_tidy_core::{Note, NoteError};
use obsidian_tidy_macros::RuleConstMetadata;
use serde::Deserialize;
use std::convert::Infallible;
use thiserror::Error;
use tracing::{instrument, trace};

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize, RuleConstMetadata)]
#[rule_metadata(
     name = "empty-content",
     description = "Rule for search notes with empty content",
     category = Category::Content
 )]
pub struct EmptyContent;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error from parser: `{0}`")]
    Parser(#[from] NoteError),

    #[error("Failed create violation: `{0}`")]
    Violation(#[from] ViolationError),
}

impl RuleRunner for EmptyContent {
    type Error = self::Error;

    #[instrument(skip(_content))]
    fn check(&self, _content: &Content, note: &Note) -> Result<Vec<Violation>, Self::Error> {
        trace!("Run check `EmptyContent`");

        if note.count_words_from_content()? == 0 {
            let violation = Violation::new("Note is empty", 1..=1)?;
            return Ok(vec![violation]);
        }

        Ok(Vec::new())
    }
}

pub fn fabric()
-> impl RuleFabric<Rule = EmptyContent, Data = EmptyContent, Error = Infallible> + Send + Sync {
    EmptyContent::fabric()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{
        DEFAULT_MOCK_VAULT, DefaultNoteGenerator, MockVaultBuilder, NoteGenerator as Generator,
    };
    use obsidian_parser::note::{NoteDefault, NoteFromReader};
    use obsidian_tidy_core::rule::{RuleConstMetadata, RuleFabric};
    use tracing_test::traced_test;

    #[derive(Default, Debug)]
    struct MyGenerator {
        have_empty_note: bool,
        default_generator: DefaultNoteGenerator,
    }

    impl Generator for MyGenerator {
        type Error = std::io::Error;

        fn generate(&mut self, file: &mut std::fs::File) -> Result<(), Self::Error> {
            if !self.have_empty_note {
                self.have_empty_note = true;
                return Ok(());
            }

            self.default_generator.generate(file)
        }
    }

    #[test]
    fn fabric() {
        let fabric = EmptyContent::fabric();

        assert_eq!(fabric.name_rule(), EmptyContent::NAME);
        assert_eq!(fabric.description_rule(), EmptyContent::DESCRIPTION);
        assert_eq!(fabric.category_rule(), EmptyContent::CATEGORY);
    }

    #[test]
    #[traced_test]
    fn empty_note() {
        let rule = EmptyContent;

        let note = Note::from_string_default("").unwrap();
        let violation = rule.check(&Content::default(), &note).unwrap();

        assert_eq!(violation.len(), 1);
    }

    #[test]
    #[traced_test]
    fn not_empty_note() {
        let rule = EmptyContent;

        let note = Note::from_string_default("Super data").unwrap();
        let violation = rule.check(&Content::default(), &note).unwrap();

        assert!(violation.is_empty());
    }

    #[test]
    #[traced_test]
    fn generated_note() {
        let rule = EmptyContent;

        let mut generator = DefaultNoteGenerator::default();
        let mut note = generator.generate_temp_note().unwrap();

        let content = Content::new(note.path());
        rule.check(&content, &Note::from_reader(note.as_file_mut()).unwrap())
            .unwrap();
    }

    #[test]
    #[traced_test]
    fn not_empty_notes() {
        let rule = EmptyContent;

        let violations = DEFAULT_MOCK_VAULT.run_rule(&rule);
        assert!(violations.is_empty());
    }

    #[test]
    #[traced_test]
    fn with_empty_notes() {
        let rule = EmptyContent;

        let mock_vault = MockVaultBuilder::<MyGenerator>::default()
            .count_notes(10)
            .build()
            .unwrap();

        let violations = mock_vault.run_rule(&rule);
        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn empty_vault() {
        let rule = EmptyContent;

        let mock_vault = MockVaultBuilder::<DefaultNoteGenerator>::default()
            .count_notes(0)
            .build()
            .unwrap();

        let violations = mock_vault.run_rule(&rule);
        assert!(violations.is_empty());
    }
}
