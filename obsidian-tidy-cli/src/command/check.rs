use super::{Cli, Runner};
use obsidian_tidy_config::Config;
use obsidian_tidy_core::rule::Content;
use rayon::prelude::*;
use std::sync::mpsc;
use tracing::{debug, instrument};

#[derive(Debug, Clone)]
pub struct RunnerCheck<'a> {
    pub(crate) content: &'a Content,
    pub(crate) config: &'a Config,
}

impl Runner for RunnerCheck<'_> {
    #[instrument]
    fn run(&self, _args: &Cli) -> anyhow::Result<()> {
        debug!("Run command `check`");

        let (sender, receiver) = mpsc::channel();

        self.config.rules().par_iter().for_each(|rule| {
            let result = rule.check(self.content);
            sender.send(result).unwrap();
        });

        for data in receiver.iter() {
            println!("{data:?}");
        }

        Ok(())
    }
}
