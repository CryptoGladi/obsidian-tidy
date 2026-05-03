use gag::BufferRedirect;
use obsidian_tidy_logger::LoggerBuilder;
use serial_test::serial;
use std::io::Read;

#[test]
#[serial]
fn console_layer_respects_filter() {
    const DEBUG_MESSAGE: &str = "DEBUG: This should be filtered out";
    const INFO_MESSAGE: &str = "INFO: This should also be filtered out";
    const WARN_MESSAGE: &str = "WARN: This should appear";

    let mut stdout_capture = BufferRedirect::stdout().expect("Failed to capture stdout");

    let guard = LoggerBuilder::default()
        .stdout(true)
        .file(false)
        .console_filter("warn")
        .expect("parse console filter")
        .build()
        .expect("build failed")
        .init()
        .expect("init failed");

    tracing::debug!(DEBUG_MESSAGE);
    tracing::info!(INFO_MESSAGE);
    tracing::warn!(WARN_MESSAGE);

    drop(guard);

    let mut captured = String::new();
    stdout_capture
        .read_to_string(&mut captured)
        .expect("Failed to read captured stdout");

    assert!(
        !captured.contains(DEBUG_MESSAGE),
        "DEBUG message should be filtered out. Captured:\n{}",
        captured
    );

    assert!(
        !captured.contains(INFO_MESSAGE),
        "INFO message should be filtered out. Captured:\n{}",
        captured
    );

    assert!(
        captured.contains(WARN_MESSAGE),
        "WARN message should appear. Captured:\n{}",
        captured
    );
}
