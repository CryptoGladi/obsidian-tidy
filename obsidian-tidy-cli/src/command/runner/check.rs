//! Run check to vault

use super::Runnable;
use crate::Cli;

#[derive(Debug)]
pub struct Check;

impl Runnable for Check {
    fn run(self, _cli: &Cli) -> anyhow::Result<()> {
        todo!()
    }
}
