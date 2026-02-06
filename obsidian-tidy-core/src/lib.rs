//! Core crate for obsidian-tidy

pub mod directories;
pub mod lint;

pub type Vault = obsidian_parser::vault::VaultInMemory;
