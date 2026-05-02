//! Run check to vault

use miette::Context;

use super::Runnable;
use crate::Cli;

#[derive(Debug)]
pub struct Check;

impl Runnable for Check {
    fn run(self, cli: &Cli) -> miette::Result<()> {
        let config = cli.config().context("invalid configuration")?;

        println!("{:?}", config);

        Ok(())
    }
}
