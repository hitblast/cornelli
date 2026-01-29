use crate::commands::{BurnCmd, KeepCmd, MailboxCmd, PathCmd};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cornelli", version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Keep a new capsule of thoughts.
    Keep(KeepCmd),
    /// Visit your mailbox.
    Mailbox(MailboxCmd),
    /// Burns the mailbox forever.
    Burn(BurnCmd),
    /// Shows the path to the mailbox core.
    Path(PathCmd),
}
