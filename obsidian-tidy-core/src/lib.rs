//! Core crate for obsidian-tidy

pub mod directories;
pub mod lint;

#[cfg(test)]
pub(crate) mod test_utils;

pub type Vault = obsidian_parser::vault::VaultInMemory;
