use obsidian_parser::prelude::*;
use rand::distr::Alphanumeric;
use rand::prelude::*;
use std::fs::{File, OpenOptions};
use std::io::Write;
use tempfile::TempDir;
use tracing::{debug, instrument};

pub trait NoteGenerator {
    type Error: std::error::Error;

    fn generate(&mut self, file: &mut File) -> Result<(), Self::Error>;
}

#[derive(Default, Debug)]
pub struct DefaultNoteGenerator {
    rng: rand::rngs::ThreadRng,
}

impl NoteGenerator for DefaultNoteGenerator {
    type Error = std::io::Error;

    fn generate(&mut self, file: &mut File) -> Result<(), Self::Error> {
        let bytes: Vec<u8> = (&mut self.rng)
            .sample_iter(Alphanumeric)
            .take(150)
            .collect();

        file.write_all(&bytes)?;
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
            let path = root.join(format!("note_{}.md", i));

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
