pub mod builder;

use obsidian_parser::prelude::*;
use std::ops::Deref;
use tempfile::TempDir;

pub use builder::{DefaultNoteGenerator, MockVaultBuilder, NoteGenerator};

pub struct MockVault<N = NoteInMemory>
where
    N: Note,
{
    #[allow(unused)]
    temp_dir: TempDir,

    vault: Vault<N>,
}

impl<N> Deref for MockVault<N>
where
    N: Note,
{
    type Target = Vault<N>;

    fn deref(&self) -> &Self::Target {
        &self.vault
    }
}

impl<N> From<MockVault<N>> for Vault<N>
where
    N: Note,
{
    fn from(value: MockVault<N>) -> Self {
        value.vault
    }
}
