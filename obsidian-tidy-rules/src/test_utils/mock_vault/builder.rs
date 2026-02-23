use lipsum::lipsum_words;
use obsidian_parser::prelude::*;
use rand::prelude::*;
use rand::rngs::ThreadRng;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::ops::Range;
use tempfile::{NamedTempFile, TempDir};
use tracing::{debug, instrument};

pub trait NoteGenerator {
    type Error: std::error::Error;

    fn generate(&mut self, file: &mut File) -> Result<(), Self::Error>;

    fn generate_temp_note(&mut self) -> Result<NamedTempFile, Self::Error>
    where
        Self::Error: From<std::io::Error>,
    {
        let mut note = NamedTempFile::new()?;
        self.generate(note.as_file_mut())?;

        Ok(note)
    }
}

#[derive(Debug)]
pub struct DefaultNoteGenerator {
    rng: ThreadRng,
    count_words: Range<usize>,
}

impl Default for DefaultNoteGenerator {
    fn default() -> Self {
        Self {
            count_words: 100..150,
            rng: ThreadRng::default(),
        }
    }
}

impl NoteGenerator for DefaultNoteGenerator {
    type Error = std::io::Error;

    fn generate(&mut self, file: &mut File) -> Result<(), Self::Error> {
        let count_worlds = self.rng.random_range(self.count_words.clone());

        let words = lipsum_words(count_worlds);
        file.write_all(words.as_bytes())?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct MockVaultBuilder<G = DefaultNoteGenerator>
where
    G: NoteGenerator,
{
    count_notes: usize,
    generator: G,
}

impl<G> Default for MockVaultBuilder<G>
where
    G: NoteGenerator + Default,
{
    fn default() -> Self {
        Self {
            count_notes: 100,
            generator: G::default(),
        }
    }
}

impl<G> MockVaultBuilder<G>
where
    G: NoteGenerator,
{
    pub fn count_notes(mut self, count_notes: usize) -> Self {
        self.count_notes = count_notes;
        self
    }

    #[allow(unused)]
    pub fn generator(mut self, generator: G) -> Self {
        self.generator = generator;
        self
    }

    #[instrument(skip(self), fields(count_notes = self.count_notes))]
    pub fn build(mut self) -> Result<super::MockVault, G::Error>
    where
        G::Error: From<std::io::Error>,
    {
        debug!("Build mock vault");

        let temp_dir = TempDir::new().unwrap();
        let root = temp_dir.path();

        for i in 1..=self.count_notes {
            let path = root.join(format!("note_{i}.md"));

            let mut file = OpenOptions::new().create_new(true).write(true).open(path)?;
            self.generator.generate(&mut file)?;
        }

        let vault_options = VaultOptions::new(root);
        let vault = VaultBuilder::new(&vault_options)
            .include_hidden(true)
            .into_iter()
            .map(|note| note.unwrap())
            .build_vault(&vault_options);

        Ok(super::MockVault { temp_dir, vault })
    }
}

#[cfg(test)]
mod tests {}
