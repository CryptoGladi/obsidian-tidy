use gag::BufferRedirect;
use obsidian_tidy_logger::LoggerBuilder;
use serial_test::serial;
use std::io::Read;

#[tokio::test]
#[serial]
async fn async_stdout() {
    let mut stdout_capture = BufferRedirect::stdout().expect("Failed to capture stdout");

    let guard = LoggerBuilder::default()
        .stdout(true)
        .file(false)
        .console_filter("info")
        .expect("parse console filter")
        .build()
        .expect("build failed")
        .init()
        .expect("init failed");

    let handles = (0..10).map(|i| {
        tokio::spawn(async move {
            tracing::info!("task {i} logging");
        })
    });
    futures::future::join_all(handles).await;

    drop(guard);

    let mut captured = String::new();
    stdout_capture
        .read_to_string(&mut captured)
        .expect("Failed to read captured stdout");

    // check data race
    for i in 0..10 {
        assert!(
            captured.contains(&format!("task {i} logging")),
            "Not found captured data. All captured: `{captured}`"
        )
    }
}
