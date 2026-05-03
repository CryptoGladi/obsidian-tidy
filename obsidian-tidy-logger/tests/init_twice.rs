use obsidian_tidy_logger::LoggerBuilder;
use serial_test::serial;
use tracing::level_filters::LevelFilter;

#[test]
#[serial]
fn init_twice_global() {
    let builder = LoggerBuilder::default()
        .stdout(true)
        .file(false)
        .console_filter(LevelFilter::WARN);

    let _guard1 = builder
        .clone()
        .build()
        .expect("build failed")
        .init()
        .expect("init failed");

    let failed_guard2 = builder.build().expect("build failed").init();

    assert!(failed_guard2.is_err());
    assert!(failed_guard2.unwrap_err().is_init());
}
