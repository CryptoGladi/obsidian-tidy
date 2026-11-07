//! A module that is needed to specify the folders that our application uses.

use directories::ProjectDirs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use tracing::{instrument, trace};

static DIRECTORIES: OnceLock<Directories> = OnceLock::new();

pub fn directories<'a>() -> &'a Directories {
    DIRECTORIES.get_or_init(Directories::new)
}

#[derive(Debug)]
pub struct Directories {
    project_dirs: ProjectDirs,
}

impl Default for Directories {
    #[expect(clippy::expect_used)]
    fn default() -> Self {
        trace!("init directories");

        let project_dirs = ProjectDirs::from("com", "CryptoGladi", "obsidian-tidy")
            .expect("no valid home directory path could be retrieved from the operating system");

        Self { project_dirs }
    }
}

impl Directories {
    #[instrument]
    pub fn new() -> Self {
        Self::default()
    }

    /// Return config dir
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use obsidian_tidy_core::directories::Directories;
    /// let directories = Directories::new();
    ///
    /// #[cfg(target_os = "linux")]
    /// assert_eq!(
    ///     directories.config_dir(),
    ///     "/home/gladi/.config/obsidian-tidy"
    /// );
    /// ```
    #[must_use]
    pub fn config_dir(&self) -> &Path {
        self.project_dirs.config_dir()
    }

    /// Return data local dir
    ///
    /// # Example
    /// ```no_run
    /// # use obsidian_tidy_core::directories::Directories;
    /// let directories = Directories::new();
    ///
    /// #[cfg(target_os = "linux")]
    /// assert_eq!(
    ///     directories.config_dir(),
    ///     "/home/gladi/.local/share/obsidian-tidy"
    /// );
    /// ```
    #[must_use]
    pub fn data_local_dir(&self) -> &Path {
        self.project_dirs.data_local_dir()
    }

    /// Return logs dirs
    ///
    /// # Example
    /// ```no_run
    /// # use obsidian_tidy_core::directories::Directories;
    /// let directories = Directories::new();
    ///
    /// #[cfg(target_os = "linux")]
    /// assert_eq!(
    ///     directories.logs_dir().to_string_lossy(),
    ///     "/home/gladi/.local/share/obsidian-tidy/logs"
    /// );
    /// ```
    #[must_use]
    pub fn logs_dir(&self) -> PathBuf {
        self.data_local_dir().join("logs")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn new() {
        Directories::new();
    }
}
