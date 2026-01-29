use async_trait::async_trait;
use clap::Args;

use crate::{
    commands::Runnable,
    core::ChristmasDB,
    log_sparkles,
    utils::{duration::parse_duration, io::clear_terminal},
};
use anyhow::Result;

#[derive(Debug, Args)]
pub struct KeepCmd {
    /// The letter to keep.
    #[arg(value_name = "LETTER_BODY")]
    body: String,

    /// The time after which the letter should be available.
    #[arg(short = 't', long = "till")]
    duration: String,
}

#[async_trait]
impl Runnable for KeepCmd {
    fn run(&self, db: &mut ChristmasDB) -> Result<()> {
        let duration = parse_duration(&self.duration)?;

        db.add_new_capsule(self.body.clone(), duration)?;
        clear_terminal();
        log_sparkles!("Text kept away~");

        Ok(())
    }
}
