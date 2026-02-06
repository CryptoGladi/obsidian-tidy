use tracing::debug;

fn main() {
    let _guard = obsidian_tidy_logging::init();

    debug!("Starting obsidian-tidy...");
}
