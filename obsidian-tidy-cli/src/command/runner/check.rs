//! Run check to vault

use miette::Context;

use super::Runnable;
use crate::Cli;

#[derive(Debug)]
pub struct Check;

impl Runnable for Check {
    fn run(self, cli: &Cli) -> miette::Result<()> {
        let _config = cli.config().context("Invalid configuration")?;

        //println!("{:?}", config);

        Ok(())
    }
}
