use std::path::Path;

#[track_caller]
pub fn find_log_file(
    dir: impl AsRef<Path>,
    prefix: &str,
    suffix: &str,
) -> Option<std::path::PathBuf> {
    std::fs::read_dir(dir)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .find(|path| {
            let filename = path.file_name().unwrap().to_string_lossy();

            filename.starts_with(prefix) && filename.ends_with(suffix)
        })
}
