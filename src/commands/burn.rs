use async_trait::async_trait;
use clap::Args;

use crate::{commands::Runnable, core::ChristmasDB};
use anyhow::Result;

#[derive(Debug, Args)]
pub struct BurnCmd;

#[async_trait]
impl Runnable for BurnCmd {
    fn run(&self, db: &mut ChristmasDB) -> Result<()> {
        db.delete()?;

        Ok(())
    }
}
