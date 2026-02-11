use crate::builder::LoggerBuilder;
use std::thread::panicking;
use tracing::{Subscriber, debug, error};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{Layer, prelude::*, registry::LookupSpan};

#[derive(Debug)]
pub struct Logger {
    _guard: WorkerGuard,
}

impl LoggerBuilder {
    fn console_layer<S>(&self) -> impl Layer<S>
    where
        S: Subscriber + for<'span> LookupSpan<'span>,
    {
        tracing_subscriber::fmt::layer()
            .with_writer(std::io::stdout)
            .with_target(false)
            .with_filter(self.filter.clone())
    }

    fn file_layer<S>(&self) -> (impl Layer<S>, WorkerGuard)
    where
        S: Subscriber + for<'span> LookupSpan<'span>,
    {
        let file_appender = tracing_appender::rolling::daily(&self.path, "");
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

        let layer = tracing_subscriber::fmt::layer()
            .compact()
            .with_ansi(false)
            .with_writer(non_blocking)
            .with_target(true)
            .with_line_number(true)
            .with_file(true)
            .with_filter(self.filter.clone());

        (layer, guard)
    }

    pub fn init(self) -> Logger {
        let registry = tracing_subscriber::registry();

        let (file_layer, guard) = self.file_layer();
        let console_layer = self.console_layer();

        registry
            .with(self.stdout.then_some(console_layer))
            .with(self.file.then_some(file_layer))
            .init();

        Logger { _guard: guard }
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        match panicking() {
            true => debug!("Done work"),
            false => error!("Panic!"),
        }
    }
}
