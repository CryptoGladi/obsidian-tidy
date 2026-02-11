//! Module for store template lints

use clap::ValueEnum;
use obsidian_tidy_core::lint::{Lints, ToggleableLint};
use obsidian_tidy_lints::ALL_LINTS;
use std::{ops::Deref, sync::LazyLock};
use tracing::{instrument, trace};

static ALL: LazyLock<Lints> = LazyLock::new(|| {
    let lints = ALL_LINTS
        .clone()
        .into_iter()
        .map(|lint| ToggleableLint::new(lint, true))
        .collect();

    Lints::new(lints).unwrap()
});

static EMPTY: LazyLock<Lints> = LazyLock::new(|| {
    let lints = ALL_LINTS
        .clone()
        .into_iter()
        .map(|lint| ToggleableLint::new(lint, false))
        .collect();

    Lints::new(lints).unwrap()
});

static STANDARD: LazyLock<Lints> = LazyLock::new(|| {
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

impl AsRef<Lints> for Template {
    #[instrument]
    fn as_ref(&self) -> &Lints {
        trace!("Template to lints");

        match self {
            Template::All => &ALL,
            Template::Standard => &STANDARD,
            Template::Empty => &EMPTY,
        }
    }
}

impl From<Template> for Lints {
    #[instrument]
    fn from(template: Template) -> Lints {
        trace!("Template to owned lints");

        match template {
            Template::All => ALL.clone(),
            Template::Standard => STANDARD.clone(),
            Template::Empty => EMPTY.clone(),
        }
    }
}

impl Deref for Template {
    type Target = Lints;

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
