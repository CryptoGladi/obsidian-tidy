//! Core crate for obsidian-tidy

#![forbid(clippy::print_stdout)]

pub mod directories;
pub mod rule;

#[cfg(test)]
pub(crate) mod test_utils;

pub type Vault = obsidian_parser::vault::Vault<Note>;

pub type Note = obsidian_parser::note::note_in_memory::NoteInMemory;
pub type NoteError = obsidian_parser::note::note_in_memory::Error;
