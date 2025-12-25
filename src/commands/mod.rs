// command use for easy access throughout the entire codebase
pub mod keep;
pub mod mailbox;

pub use keep::KeepCmd;
pub use mailbox::MailboxCmd;

use anyhow::Result;
use async_trait::async_trait;

use crate::core::ChristmasDB;

/// Trait for all runnable commands.
#[async_trait]
pub trait Runnable {
    async fn run(&self, db: &mut ChristmasDB) -> Result<()>;
}
