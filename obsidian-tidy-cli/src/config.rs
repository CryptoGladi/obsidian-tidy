use super::Cli;
use miette::{Context, Diagnostic, IntoDiagnostic, NamedSource, SourceOffset, SourceSpan};
use obsidian_tidy_config::{Config, Error as ConfigError};
use obsidian_tidy_rules::ALL_RULES_FABRICS;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error, Diagnostic)]
#[error("Failed config load")]
pub struct Miette {
    #[source]
    source: ConfigError,

    #[source_code]
    input: NamedSource<String>,

    #[label("{source}")]
    location: Option<SourceSpan>,
}

impl Miette {
    pub fn from(input: impl Into<String>, path: impl AsRef<Path>, error: ConfigError) -> Self {
        let source: String = input.into();
        let path_str = path.as_ref().to_string_lossy().into_owned();

        let location = if let ConfigError::Json(ref json) = error {
            let offset = SourceOffset::from_location(&source, json.line(), json.column());
            Some(SourceSpan::new(offset, 0))
        } else {
            None
        };

        Self {
            input: NamedSource::new(path_str, source).with_language("Json"),
            source: error,
            location,
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
            .map_err(|error| Miette::from(data, path, error))?;

        Ok(config)
    }
}
