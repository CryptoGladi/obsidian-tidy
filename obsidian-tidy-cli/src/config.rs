use super::Cli;
use miette::{Context, Diagnostic, IntoDiagnostic, NamedSource, SourceOffset, SourceSpan};
use obsidian_tidy_config::{Config, Error as ConfigError};
use obsidian_tidy_rules::ALL_RULES_FABRICS;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("Malformed JSON")]
pub struct SerdeError {
    cause: serde_json::Error,

    #[source_code]
    input: NamedSource<String>,

    #[label("{cause}")]
    location: SourceSpan,
}

impl SerdeError {
    pub fn from_serde_error(
        input: impl Into<String>,
        path: impl AsRef<Path>,
        cause: serde_json::Error,
    ) -> Self {
        let source = input.into();
        let path = path.as_ref();

        let offset = SourceOffset::from_location(&source, cause.line(), cause.column());
        let span = SourceSpan::new(offset, 0);

        Self {
            cause,
            input: NamedSource::new(path.to_string_lossy(), source),
            location: span,
        }
    }
}

impl Cli {
    #[must_use]
    pub fn config_path(&self) -> PathBuf {
        self.path.join(".obsidian-tidy.json")
    }

    pub fn config(&self) -> miette::Result<Config> {
        let path = self.config_path();

        let data = std::fs::read_to_string(&path)
            .into_diagnostic()
            .with_context(|| format!("Failed to read file: `{}`", path.display()))?;

        let config = Config::loader(&ALL_RULES_FABRICS)
            .load(&mut data.as_bytes())
            .map_err(|cause| {
                let ConfigError::Json(cause) = cause else {
                    unreachable!("expected Json error, got: {:?}", cause);
                };

                SerdeError::from_serde_error(data, path, cause)
            })?;

        Ok(config)
    }
}
