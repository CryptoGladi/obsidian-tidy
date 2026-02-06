use clap::Parser;
use std::path::PathBuf;

fn default_path() -> PathBuf {
    std::env::current_dir().unwrap()
}

#[derive(Debug, Parser)]
#[command(name = "obsidian-tidy")]
#[command(
    version,
    about = "Blazingly fast Obsidian vault linter",
    long_about = "ds"
)]
pub struct Args {
    #[arg(long)]
    path: PathBuf,
}
