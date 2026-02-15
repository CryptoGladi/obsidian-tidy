//! Rule for search notes with empty content

use obsidian_parser::prelude::*;
use obsidian_tidy_core::rule::{Category, Content, Rule, Violation};
use std::convert::Infallible;
use tracing::{debug, instrument};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EmptyContent;

impl Rule for EmptyContent {
    type Error = Infallible;

    fn name(&self) -> &str {
        "empty-content"
    }

    fn description(&self) -> &str {
        "Rule for search notes with empty content"
    }

    fn category(&self) -> Category {
        Category::Content
    }

    #[instrument(skip(content))]
    fn check(&self, content: &Content) -> Result<Vec<Violation>, Self::Error> {
        debug!("Run check `EmptyContent`");

        let violation = content
            .vault
            .notes()
            .iter()
            .filter(|note| note.count_words_from_content().unwrap() == 0)
            .map(|note| Violation::new("Empty note", note.path().unwrap(), 0..1).unwrap())
            .collect();

        Ok(violation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{
        DEFAULT_MOCK_VAULT, DefaultNoteGenerator, MockVaultBuilder, NoteGenerator as Generator,
    };
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
    fn not_empty_notes() {
        let rule = EmptyContent::default();

        let violation = rule
            .check(&Content {
                vault: DEFAULT_MOCK_VAULT.clone(),
            })
            .unwrap();

        assert!(violation.is_empty());
    }

    #[test]
    #[traced_test]
    fn with_empty_note() {
        let rule = EmptyContent::default();

        let mock_vault = MockVaultBuilder::<MyGenerator>::default()
            .count_notes(10)
            .build()
            .unwrap();

        let violations = rule
            .check(&Content {
                vault: mock_vault.into(),
            })
            .unwrap();

        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn empty_vault() {
        let rule = EmptyContent::default();

        let mock_vault = MockVaultBuilder::<DefaultNoteGenerator>::default()
            .count_notes(0)
            .build()
            .unwrap();

        let violations = rule
            .check(&Content {
                vault: mock_vault.into(),
            })
            .unwrap();

        assert!(violations.is_empty());
    }
}
