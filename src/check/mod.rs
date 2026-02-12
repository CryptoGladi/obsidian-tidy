use obsidian_tidy_core::lint::{Content, Lints};
use rayon::prelude::*;
use std::sync::mpsc;

/*

pub struct Check {
    lints: Lints,
    content: Content,
}

impl Check {
    pub fn run(&self) {
        let (sender, receiver) = mpsc::channel();

        self.lints.par_iter().for_each(|lint| {
            let result = lint.check(&self.content);
        });
    }
}
*/
