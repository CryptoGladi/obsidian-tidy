//! Rule for search notes with empty content

use obsidian_parser::note::Note as _;
use obsidian_tidy_core::rule::violation::{Error as ViolationError, Violation};
use obsidian_tidy_core::rule::{Category, Content, Rule, RuleFabric};
use obsidian_tidy_core::{Note, NoteError};
use serde::Deserialize;
use std::convert::Infallible;
use thiserror::Error;
use tracing::{instrument, trace};

const NAME: &str = "empty-content";
const DESCRIPTION: &str = "Rule for search notes with empty content";
const CATEGORY: Category = Category::Content;

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
pub struct EmptyContent;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error from parser: `{0}`")]
    Parser(#[from] NoteError),

    #[error("Failed create violation: `{0}`")]
    Violation(#[from] ViolationError),
}

impl Rule for EmptyContent {
    type Error = self::Error;

    fn name(&self) -> &'static str {
        self::NAME
    }

    fn description(&self) -> &'static str {
        self::DESCRIPTION
    }

    fn category(&self) -> Category {
        self::CATEGORY
    }

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmptyContentFabric;

impl RuleFabric for EmptyContentFabric {
    type Rule = EmptyContent;
    type Data = EmptyContent;
    type Error = Infallible;

    fn name_rule(&self) -> &str {
        self::NAME
    }

    fn description_rule(&self) -> &str {
        self::DESCRIPTION
    }

    fn category_rule(&self) -> Category {
        self::CATEGORY
    }

    fn create_rule(&self, data: Self::Data) -> Result<Self::Rule, Self::Error> {
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{
        DEFAULT_MOCK_VAULT, DefaultNoteGenerator, MockVaultBuilder, NoteGenerator as Generator,
    };
    use obsidian_parser::note::{NoteDefault, NoteFromReader};
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
