mod common;

use common::find_log_file;
use obsidian_tidy_logger::LoggerBuilder;
use serial_test::serial;
use tempfile::TempDir;

#[test]
#[serial]
fn file_layer_respects_filter() {
    const DEBUG_MESSAGE: &str = "DEBUG: This should be filtered out";
    const INFO_MESSAGE: &str = "INFO: This should also be filtered out";
    const WARN_MESSAGE: &str = "WARN: This should appear";

    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let log_path = temp_dir.path().to_path_buf();

    let guard = LoggerBuilder::default()
        .path(&log_path)
        .stdout(false)
        .file(true)
        .file_filter("warn")
        .expect("parse file filter")
        .filename_prefix("test-logger")
        .filename_suffix("log")
        .build()
        .expect("build failed")
        .init()
        .expect("init failed");

    tracing::debug!(DEBUG_MESSAGE);
    tracing::info!(INFO_MESSAGE);
    tracing::warn!(WARN_MESSAGE);

    drop(guard);

    let log_file =
        find_log_file(&log_path, "test-logger", "log").expect("Log file should be created");

    let content = std::fs::read_to_string(&log_file).expect("Failed to read log file");

    assert!(
        !content.contains(DEBUG_MESSAGE),
        "DEBUG message should be filtered out. File content:\n{}",
        content
    );

    assert!(
        !content.contains(INFO_MESSAGE),
        "INFO message should be filtered out. File content:\n{}",
        content
    );

    assert!(
        content.contains(WARN_MESSAGE),
        "WARN message should appear. File content:\n{}",
        content
    );
}
