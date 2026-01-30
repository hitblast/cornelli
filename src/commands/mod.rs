// command use for easy access throughout the entire codebase
mod burn;
mod keep;
mod mailbox;

pub use burn::BurnCmd;
pub use keep::KeepCmd;
pub use mailbox::MailboxCmd;

use anyhow::Result;
use async_trait::async_trait;

use crate::core::ChristmasDB;

/// Trait for all runnable commands.
#[async_trait]
pub trait Runnable {
    fn run(&self, db: &mut ChristmasDB) -> Result<()>;
}
