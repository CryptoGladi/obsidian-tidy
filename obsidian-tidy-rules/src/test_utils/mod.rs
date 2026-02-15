pub mod mock_vault;

use std::sync::LazyLock;

pub use mock_vault::{DefaultNoteGenerator, MockVault, MockVaultBuilder, NoteGenerator};

pub static DEFAULT_MOCK_VAULT: LazyLock<MockVault> = LazyLock::new(|| {
    MockVaultBuilder::<DefaultNoteGenerator>::default()
        .build()
        .unwrap()
});
