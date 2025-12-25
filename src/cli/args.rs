use crate::commands::{KeepCmd, MailboxCmd};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "cornelli", version, about)]
pub struct Args {
    /// Accepts all interactive prompts.
    #[arg(short = 'y', long, global = true)]
    pub accept_all: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Keep a new capsule of thoughts.
    Keep(KeepCmd),
    /// Visit your mailbox.
    Mailbox(MailboxCmd),
}
