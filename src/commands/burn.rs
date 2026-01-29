use async_trait::async_trait;
use clap::Args;

use crate::{commands::Runnable, core::ChristmasDB, log_orb, utils::io::clear_terminal};
use anyhow::Result;

#[derive(Debug, Args)]
pub struct BurnCmd;

#[async_trait]
impl Runnable for BurnCmd {
    fn run(&self, db: &mut ChristmasDB) -> Result<()> {
        clear_terminal();
        db.delete()?;
        log_orb!("Forever burnt away...");

        Ok(())
    }
}
