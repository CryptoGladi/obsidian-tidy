//! A module that is needed to specify the folders that our application uses.

use directories::ProjectDirs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

static DIRECTORIES: OnceLock<Directories> = OnceLock::new();

pub fn directories<'a>() -> &'a Directories {
    DIRECTORIES.get_or_init(|| Directories::new())
}

pub struct Directories {
    project_dirs: ProjectDirs,
}

impl Directories {
    pub fn new() -> Self {
        if let Some(project_dirs) = ProjectDirs::from("com", "CryptoGladi", "obsidian-tidy") {
            return Directories { project_dirs };
        }

        panic!("No valid home directory path could be retrieved from the operating system");
    }

    /// Return config dir
    ///
    /// # Example
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
    ///     directories.config_dir(),
    ///     "/home/gladi/.config/obsidian-tidy/logs"
    /// );
    /// ```
    pub fn logs_dir(&self) -> PathBuf {
        self.data_local_dir().join("logs")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        Directories::new();
    }
}
