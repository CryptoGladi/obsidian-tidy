//! Errors for Config

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: `{0}`")]
    IO(#[from] std::io::Error),

    #[error("Json error: `{0}`")]
    Json(#[from] serde_json::Error),
}
