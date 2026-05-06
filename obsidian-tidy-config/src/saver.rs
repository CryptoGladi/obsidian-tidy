//! Serialization logic for saving configuration to output.
//!
//! This module provides a simple, zero-copy serializer that writes [`Config`]
//! as pretty-printed JSON.

use super::{Config, Error};
use std::io::Write;
use tracing::instrument;

/// A saver for serializing [`Config`] to output.
///
/// Created via [`Config::saver()`]. Borrows the config and writes JSON
/// to any [`std::io::Write`] implementor.
#[derive(Debug)]
pub struct ConfigSaver<'a> {
    pub(crate) config: &'a Config,
    pretty: bool,
}

impl ConfigSaver<'_> {
    /// Creates a new [`ConfigSaver`] for the given config.
    ///
    /// This is an internal constructor; users should prefer
    /// [`Config::saver()`] for ergonomic API access.
    pub(crate) const fn new(config: &Config) -> ConfigSaver<'_> {
        ConfigSaver {
            config,
            pretty: false,
        }
    }

    /// Configures whether to output pretty-printed JSON.
    ///
    /// # Arguments
    ///
    /// * `pretty` — If `true`, output includes indentation and newlines
    ///   for human readability. If `false` (default), output is compact
    ///   for minimal size and faster parsing.
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_config::{Config, Template};
    ///
    /// let config = Config::new(Template::All);
    ///
    /// let mut output = Vec::new();
    /// config.saver()
    ///     .pretty(true)  // Enable pretty-printing
    ///     .save(&mut output)?;
    ///
    /// let json = String::from_utf8(output)?;
    /// assert!(json.contains('\n')); // Pretty output has newlines
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub const fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = pretty;
        self
    }

    /// Saves the configuration to the given writer as pretty-printed JSON.
    ///
    /// # Example
    ///
    /// ```
    /// use obsidian_tidy_config::{Template, Config};
    ///
    /// let config = Config::new(Template::All);
    ///
    /// let mut buffer = Vec::new();
    /// config.saver().save(&mut buffer).unwrap();
    /// ```
    #[instrument(skip(writer), level = "debug", err)]
    pub fn save(&self, mut writer: impl Write) -> Result<(), Error> {
        if self.pretty {
            serde_json::to_writer_pretty(&mut writer, self.config)?;
        } else {
            serde_json::to_writer(&mut writer, self.config)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Template;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn save_compact_json() {
        let config = Config::new(Template::Empty);
        let mut buffer = Vec::new();

        config.saver().save(&mut buffer).expect("save failed");

        let output = String::from_utf8(buffer).expect("invalid utf8");

        // Compact JSON should not contain unnecessary whitespace
        assert!(output.starts_with('{'));
        assert!(!output.contains("\n"));
    }

    #[test]
    #[traced_test]
    fn save_pretty_json() {
        let config = Config::new(Template::Empty);
        let mut buffer = Vec::new();

        config
            .saver()
            .pretty(true)
            .save(&mut buffer)
            .expect("save failed");

        let output = String::from_utf8(buffer).expect("invalid utf8");

        // Pretty JSON should have formatting
        assert!(output.contains('\n'));
        assert!(output.contains("  "));
    }
}
