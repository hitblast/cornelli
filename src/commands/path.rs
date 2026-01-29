use async_trait::async_trait;
use clap::Args;

use crate::{commands::Runnable, core::ChristmasDB};
use anyhow::Result;

#[derive(Debug, Args)]
pub struct PathCmd;

#[async_trait]
impl Runnable for PathCmd {
    fn run(&self, db: &mut ChristmasDB) -> Result<()> {
        println!("{}", db.path().display());

        Ok(())
    }
}
