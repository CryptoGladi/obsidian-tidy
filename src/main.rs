mod args;

use clap::Parser;
use tracing::debug;

fn main() {
    let _guard = obsidian_tidy_logging::init();

    let args = args::Args::parse();

    debug!("Starting obsidian-tidy with args: {:?}", args);
}
