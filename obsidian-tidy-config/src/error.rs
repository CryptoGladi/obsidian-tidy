//! Error types for configuration operations.
//!
//! This module defines the [`Error`] enum, which consolidates all fallible
//! outcomes from loading, saving, or validating configuration.
//!
//! [`Error`]: crate::Error

use derive_more::IsVariant;
use thiserror::Error;

#[derive(Debug, Error, IsVariant)]
#[non_exhaustive]
pub enum Error {
    #[error("IO error: `{0}`")]
    IO(#[from] std::io::Error),

    #[error("Json error: `{0}`")]
    Json(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::de::Error as _;

    #[test]
    fn is_enum() {
        let io_error = Error::IO(std::io::Error::other("Oh no"));
        assert!(io_error.is_io());
        assert!(!io_error.is_json());

        let json_error = Error::Json(serde_json::Error::custom("No"));
        assert!(!json_error.is_io());
        assert!(json_error.is_json());
    }
}
