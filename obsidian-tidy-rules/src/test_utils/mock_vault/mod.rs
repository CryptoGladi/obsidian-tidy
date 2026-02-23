pub mod builder;

use obsidian_tidy_core::Vault;
use obsidian_tidy_core::rule::{Content, Rule, Violation};
use std::ops::Deref;
use tempfile::TempDir;

pub use builder::{DefaultNoteGenerator, MockVaultBuilder, NoteGenerator};

pub struct MockVault {
    #[allow(unused)]
    temp_dir: TempDir,

    vault: Vault,
}

impl MockVault {
    pub fn run_rule<R>(&self, rule: &R) -> Vec<Violation>
    where
        R: Rule,
    {
        let content = Content::from(self.vault.clone());

        self.notes()
            .iter()
            .flat_map(|note| rule.check(&content, note).unwrap())
            .collect()
    }
}

impl Deref for MockVault {
    type Target = Vault;

    fn deref(&self) -> &Self::Target {
        &self.vault
    }
}

impl AsRef<Vault> for MockVault {
    fn as_ref(&self) -> &Vault {
        &self.vault
    }
}

impl From<MockVault> for Vault {
    fn from(mock_vault: MockVault) -> Self {
        mock_vault.vault
    }
}
