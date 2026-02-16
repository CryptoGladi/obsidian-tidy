use crate::Vault;
use obsidian_parser::prelude::*;
use std::path::Path;

#[derive(Debug, Default, Clone)]
pub struct Content {
    pub vault: Vault,
}

impl Content {
    pub fn new(path: impl AsRef<Path>) -> Self {
        let options = VaultOptions::new(path);

        let vault = VaultBuilder::new(&options)
            .into_iter()
            .filter_map(Result::ok)
            .build_vault(&options);

        Self { vault }
    }
}
