use obsidian_tidy_core::rule::{Content, Rules, SharedErrorRule};
use rayon::prelude::*;
use std::sync::mpsc;

pub struct Check {
    rules: Rules<SharedErrorRule>,
    content: Content,
}

impl Check {
    pub fn run(&self) {
        let (sender, receiver) = mpsc::channel();

        self.rules.par_iter().for_each(|rule| {
            let result = rule.check(&self.content);
            sender.send(result.unwrap());
        });
    }
}
