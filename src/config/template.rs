//! Module for store template lints

use clap::ValueEnum;
use obsidian_tidy_core::lint::{Lints, ToggleableLint, WrappedAnyhowError};
use obsidian_tidy_lints::ALL_LINTS;
use std::{ops::Deref, sync::LazyLock};
use tracing::{instrument, trace};

static ALL: LazyLock<Lints<WrappedAnyhowError>> = LazyLock::new(|| {
    let lints = ALL_LINTS
        .clone()
        .into_iter()
        .map(|lint| ToggleableLint::new(lint.into(), true))
        .collect();

    Lints::new(lints).unwrap()
});

static EMPTY: LazyLock<Lints<WrappedAnyhowError>> = LazyLock::new(|| {
    let lints = ALL_LINTS
        .clone()
        .into_iter()
        .map(|lint| ToggleableLint::new(lint.into(), false))
        .collect();

    Lints::new(lints).unwrap()
});

static STANDARD: LazyLock<Lints<WrappedAnyhowError>> = LazyLock::new(|| {
    let mut lints = EMPTY.clone();

    lints["test-lint"].enable();

    lints
});

/// Template config
#[derive(Debug, Clone, ValueEnum)]
#[clap(rename_all = "kebab-case")]
pub enum Template {
    /// Enabled all lints
    All,

    /// Standard config.
    /// Recommended for most users
    Standard,

    /// Disabled all lints
    Empty,
}

impl AsRef<Lints<WrappedAnyhowError>> for Template {
    #[instrument]
    fn as_ref(&self) -> &Lints<WrappedAnyhowError> {
        trace!("Template to lints");

        match self {
            Template::All => &ALL,
            Template::Standard => &STANDARD,
            Template::Empty => &EMPTY,
        }
    }
}

impl From<Template> for Lints<WrappedAnyhowError> {
    #[instrument]
    fn from(template: Template) -> Lints<WrappedAnyhowError> {
        trace!("Template to owned lints");

        match template {
            Template::All => ALL.clone(),
            Template::Standard => STANDARD.clone(),
            Template::Empty => EMPTY.clone(),
        }
    }
}

impl Deref for Template {
    type Target = Lints<WrappedAnyhowError>;

    fn deref(&self) -> &Self::Target {
        match self {
            Template::All => &ALL,
            Template::Standard => &STANDARD,
            Template::Empty => &EMPTY,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Template;
    use obsidian_tidy_lints::ALL_LINTS;

    #[test]
    fn all_check() {
        assert_eq!(Template::All.len(), ALL_LINTS.len());
        assert!(Template::All.iter().all(|lint| lint.enabled()));
    }

    #[test]
    fn empty_check() {
        assert_eq!(Template::Empty.len(), ALL_LINTS.len());
        assert!(Template::Empty.iter().all(|lint| lint.disabled()));
    }

    #[test]
    fn standart() {
        assert_eq!(Template::Standard.len(), ALL_LINTS.len());

        assert!(Template::All.iter().any(|lint| lint.enabled()));
        assert!(Template::Empty.iter().any(|lint| lint.disabled()));
    }
}
