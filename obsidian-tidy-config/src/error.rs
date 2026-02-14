//! Errors for Config

use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: `{0}`")]
    IO(#[from] std::io::Error),

    #[error("Toml deserialize error `{0}`")]
    Deserialize(#[from] toml::de::Error),

    #[error("Toml serialize error `{0}`")]
    Serialize(#[from] toml::ser::Error),

    #[error("Config file already exists")]
    AlreadyExists(PathBuf),
}
