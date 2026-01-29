use async_trait::async_trait;
use clap::Args;

use crate::{commands::Runnable, core::ChristmasDB, log_sparkles, utils::duration::parse_duration};
use anyhow::Result;

#[derive(Debug, Args)]
pub struct KeepCmd {
    /// The text to keep.
    text: String,

    /// The duration to keep for.
    #[arg(short, long)]
    duration: String,
}

#[async_trait]
impl Runnable for KeepCmd {
    fn run(&self, db: &mut ChristmasDB) -> Result<()> {
        let duration = parse_duration(&self.duration)?;

        db.add_new_capsule(self.text.clone(), duration)?;
        log_sparkles!("Text kept away~");

        Ok(())
    }
}
